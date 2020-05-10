package http

import (
	"context"
	"encoding/json"
	"errors"
	"github.com/go-kit/kit/endpoint"
	"net/http"
	"swa-semestral-project/cart_order_service/order"
	"swa-semestral-project/cart_order_service/order/transport"

	"github.com/go-kit/kit/log"
	kithandler "github.com/go-kit/kit/transport"
	kithttp "github.com/go-kit/kit/transport/http"
	"github.com/gorilla/mux"
)

var (
	ErrBadRouting = errors.New("bad routing")
)

// NewService wires Go kit endpoints to the HTTP transport.
func NewService(
	svcEndpoints transport.Endpoints, options []kithttp.ServerOption, logger log.Logger,
) http.Handler {
	// set-up router and initialize http endpoints
	var (
		r            = mux.NewRouter()
		errorLogger  = kithttp.ServerErrorHandler(kithandler.NewLogErrorHandler(logger))
		errorEncoder = kithttp.ServerErrorEncoder(encodeErrorResponse)
	)
	options = append(options, errorLogger, errorEncoder)
	//options := []kithttp.ServerOption{
	//	kithttp.ServerErrorLogger(logger),
	//	kithttp.ServerErrorEncoder(encodeError),
	//}
	// HTTP Post - /orders
	r.Methods("POST").Path("/orders").Handler(kithttp.NewServer(
		svcEndpoints.Create,
		decodeCreateRequest,
		encodeResponse,
		options...,
	))

	// HTTP Get - /orders/{id}
	r.Methods("GET").Path("/orders/{id}").Handler(kithttp.NewServer(
		svcEndpoints.GetByID,
		decodeGetByIDRequest,
		encodeResponse,
		options...,
	))

	// HTTP Get - /orders/{id}
	r.Methods("GET").Path("/orders/customer/{id}").Handler(kithttp.NewServer(
		svcEndpoints.GetOrdersByCustomerID,
		decodeGetByIDRequest,
		encodeResponse,
		options...,
	))

	// HTTP Post - /orders/status
	r.Methods("POST").Path("/orders/status").Handler(kithttp.NewServer(
		svcEndpoints.ChangeStatus,
		decodeChangeStausRequest,
		encodeResponse,
		options...,
	))

	// HTTP Post - /orders/status
	r.Methods("POST").Path("/orders/cancel/{id}").Handler(kithttp.NewServer(
		svcEndpoints.Cancel,
		decodeCancelRequest,
		encodeResponse,
		options...,
	))
	return r
}

func decodeCreateRequest(_ context.Context, r *http.Request) (request interface{}, err error) {
	var req transport.CreateRequest
	if e := json.NewDecoder(r.Body).Decode(&req.Order); e != nil {
		return nil, e
	}
	return req, nil
}

func decodeGetByIDRequest(_ context.Context, r *http.Request) (request interface{}, err error) {
	vars := mux.Vars(r)
	id, ok := vars["id"]
	if !ok {
		return nil, ErrBadRouting
	}
	return transport.GetByIDRequest{ID: id}, nil
}

func decodeChangeStausRequest(_ context.Context, r *http.Request) (request interface{}, err error) {
	var req transport.ChangeStatusRequest
	if e := json.NewDecoder(r.Body).Decode(&req); e != nil {
		return nil, e
	}
	return req, nil
}

func decodeCancelRequest(_ context.Context, r *http.Request) (request interface{}, err error) {
	vars := mux.Vars(r)
	id, ok := vars["id"]
	if !ok {
		return nil, ErrBadRouting
	}
	return transport.CancelRequest{ID: id}, nil
}

func encodeResponse(ctx context.Context, w http.ResponseWriter, response interface{}) error {
	if f, ok := response.(endpoint.Failer); ok && f.Failed() != nil {
		// Not a Go kit transport error, but a business-logic error.
		// Provide those as HTTP errors.
		encodeErrorResponse(ctx, f.Failed(), w)
		return nil
	}
	w.Header().Set("Content-Type", "application/json; charset=utf-8")
	return json.NewEncoder(w).Encode(response)
}

func encodeErrorResponse(_ context.Context, err error, w http.ResponseWriter) {
	if err == nil {
		panic("encodeError with nil error")
	}
	w.Header().Set("Content-Type", "application/json; charset=utf-8")
	w.WriteHeader(codeFrom(err))
	json.NewEncoder(w).Encode(map[string]interface{}{
		"error": err.Error(),
	})
}

func codeFrom(err error) int {
	switch err {
	case order.ErrOrderNotFound:
		return http.StatusBadRequest
	default:
		return http.StatusInternalServerError
	}
}
