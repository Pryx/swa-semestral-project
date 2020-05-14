package http

import (
	"bytes"
	"database/sql"
	"encoding/json"
	"fmt"
	kitlog "github.com/go-kit/kit/log"
	kithttp "github.com/go-kit/kit/transport/http"
	"github.com/stretchr/testify/assert"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"net/http"
	"net/http/httptest"
	"swa-semestral-project/cart_order_service/order"
	ordersvc "swa-semestral-project/cart_order_service/order/implementation"
	"time"

	"github.com/stretchr/testify/mock"
	"swa-semestral-project/cart_order_service/order/mocks"
	"swa-semestral-project/cart_order_service/order/transport"
	"testing"
)

func NewHandler(mockRepo order.Repository) http.Handler {
	svc := ordersvc.NewService(mockRepo, kitlog.NewNopLogger())
	endpoints := transport.MakeEndpoints(svc)
	handler := NewService(endpoints, []kithttp.ServerOption{}, kitlog.NewNopLogger())
	return handler
}

func MockRepo() *mocks.Repository {
	mockRepo := &mocks.Repository{}
	mockRepo.On("CreateOrder",
		mock.AnythingOfType("context.valueCtx"),
		mock.AnythingOfType("order.Order")).Return("12345", nil)
	dOrder := DummyOrder()

	mockRepo.On("GetOrdersByCustomerID",
		mock.AnythingOfType("context.valueCtx"),
		"C12345").Return(dOrder, nil)

	mockRepo.On("GetOrdersByCustomerID",
		mock.AnythingOfType("context.valueCtx"),
		"C73845").Return(nil, sql.ErrNoRows)

	mockRepo.On("ChangeOrderStatus",
		mock.AnythingOfType("context.valueCtx"),
		"12345").Return(nil)

	mockRepo.On("ChangeOrderStatus",
		mock.AnythingOfType("context.valueCtx"),
		"1234").Return(sql.ErrNoRows)

	return mockRepo
}

func DummyOrder() order.Order {
	uid, _ := primitive.ObjectIDFromHex("12345")
	return order.Order{
		ID:         uid,
		CustomerID: "C12345",
		Status:     "Pending",
		CreatedOn:  time.Now().Unix(),
		Address:    "Test adress",
		Products:   []string{"ASD123", "FOO123"},
	}
}

type Errorer struct {
	Err string `json:"error"`
}

func TestGetOrderByIDSThatExist(t *testing.T) {
	mockRepo := &mocks.Repository{}
	dOrder := DummyOrder()
	mockRepo.On("GetOrderByID",
		mock.Anything,
		"12345").Return(dOrder, nil)

	h := NewHandler(mockRepo)

	r, _ := http.NewRequest("GET", "/orders/12345", nil)
	w := httptest.NewRecorder()

	h.ServeHTTP(w, r)
	assert.Equal(t, http.StatusOK, w.Code)

	res := transport.GetByIDResponse{}
	err := json.Unmarshal(w.Body.Bytes(), &res)
	if err != nil {
		fmt.Println(err)
	}
	assert.Equal(t, dOrder, res.Order)
	assert.Equal(t, nil, res.Err)
}

func TestGetOrderByIDSThatDoesNotExist(t *testing.T) {
	mockRepo := &mocks.Repository{}
	mockRepo.On("GetOrderByID",
		mock.Anything,
		"1234").Return(order.Order{}, sql.ErrNoRows)

	h := NewHandler(mockRepo)

	r, _ := http.NewRequest("GET", "/orders/1234", nil)
	w := httptest.NewRecorder()

	h.ServeHTTP(w, r)
	assert.Equal(t, http.StatusBadRequest, w.Code)

	res := Errorer{}
	err := json.Unmarshal(w.Body.Bytes(), &res)
	if err != nil {
		fmt.Println(err)
	}
	assert.Equal(t, order.ErrOrderNotFound.Error(), res.Err)
}

func TestGetOrderByCustomerIDSThatExist(t *testing.T) {
	mockRepo := &mocks.Repository{}
	dOrder := DummyOrder()
	mockRepo.On("GetOrdersByCustomerID",
		mock.Anything,
		"C12345").Return([]order.Order{dOrder}, nil)

	h := NewHandler(mockRepo)

	r, _ := http.NewRequest("GET", "/orders/customer/C12345", nil)
	w := httptest.NewRecorder()

	h.ServeHTTP(w, r)
	assert.Equal(t, http.StatusOK, w.Code)

	res := transport.GetOrdersByCustomerIDResponse{}
	err := json.Unmarshal(w.Body.Bytes(), &res)
	if err != nil {
		fmt.Println(err)
	}
	assert.Equal(t, []order.Order{dOrder}, res.Orders)
	assert.Equal(t, nil, res.Err)
}

func TestGetOrderByCustomerIDSThatDoesNotExist(t *testing.T) {
	mockRepo := &mocks.Repository{}
	mockRepo.On("GetOrdersByCustomerID",
		mock.Anything,
		"C73845").Return(nil, sql.ErrNoRows)

	h := NewHandler(mockRepo)

	r, _ := http.NewRequest("GET", "/orders/customer/C73845", nil)
	w := httptest.NewRecorder()

	h.ServeHTTP(w, r)
	assert.Equal(t, http.StatusBadRequest, w.Code)

	res := Errorer{}
	err := json.Unmarshal(w.Body.Bytes(), &res)
	if err != nil {
		fmt.Println(err)
	}
	assert.Equal(t, order.ErrOrderNotFound.Error(), res.Err)
}

func TestChangeOrderStatus(t *testing.T) {
	mockRepo := &mocks.Repository{}
	mockRepo.On("ChangeOrderStatus",
		mock.Anything,
		"12345",
		mock.AnythingOfType("string")).Return(nil)

	h := NewHandler(mockRepo)

	var jsonStr = []byte(`{"id": "12345", "status": "Accepted"}`)
	r, _ := http.NewRequest("POST", "/orders/status", bytes.NewBuffer(jsonStr))
	w := httptest.NewRecorder()

	h.ServeHTTP(w, r)
	assert.Equal(t, http.StatusOK, w.Code)

	res := transport.GetByIDResponse{}
	err := json.Unmarshal(w.Body.Bytes(), &res)
	if err != nil {
		fmt.Println(err)
	}
	assert.Equal(t, nil, res.Err)
}

func TestChangeOrderStatusOrderDOesNOtExist(t *testing.T) {
	mockRepo := &mocks.Repository{}
	mockRepo.On("ChangeOrderStatus",
		mock.Anything,
		"1234",
		mock.AnythingOfType("string")).Return(sql.ErrNoRows)

	h := NewHandler(mockRepo)

	var jsonStr = []byte(`{"id": "1234", "status": "Delivered"}`)
	r, _ := http.NewRequest("POST", "/orders/status", bytes.NewBuffer(jsonStr))
	w := httptest.NewRecorder()

	h.ServeHTTP(w, r)
	assert.Equal(t, http.StatusBadRequest, w.Code)

	res := Errorer{}
	err := json.Unmarshal(w.Body.Bytes(), &res)
	if err != nil {
		fmt.Println(err)
	}
	assert.Equal(t, order.ErrOrderNotFound.Error(), res.Err)
}

func TestCancelOrder(t *testing.T) {
	mockRepo := &mocks.Repository{}
	mockRepo.On("ChangeOrderStatus",
		mock.Anything,
		"12345",
		mock.AnythingOfType("string")).Return(nil)

	h := NewHandler(mockRepo)

	r, _ := http.NewRequest("POST", "/orders/cancel/12345", nil)
	w := httptest.NewRecorder()

	h.ServeHTTP(w, r)
	assert.Equal(t, http.StatusOK, w.Code)

	res := transport.GetByIDResponse{}
	err := json.Unmarshal(w.Body.Bytes(), &res)
	if err != nil {
		fmt.Println(err)
	}
	assert.Equal(t, nil, res.Err)
}
