package implementation

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"github.com/go-kit/kit/log/level"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"net/http"
	"os"
	"swa-semestral-project/cart_order_service/order"
	"sync"

	"github.com/go-kit/kit/log"
)

type Data struct {
	URL      string
	Response *http.Response
	Err      error
}

type repository struct {
	db     *mongo.Database
	logger log.Logger
}

// NewRepository returns a concrete repository backed by CockroachDB
func NewRepository(db *mongo.Database, logger log.Logger) (order.Repository, error) {
	// return  repository
	return &repository{
		db:     db,
		logger: log.With(logger, "rep", "mongodb"),
	}, nil
}

// CreateOrder inserts a new order and its order items into db
func (repo *repository) CreateOrder(ctx context.Context, order order.Order) (string, error) {

	port, exists := os.LookupEnv("PRODUCT_SERVICE_PORT")
	if !exists {
		return "", errors.New("missing Product service port")
	}

	url := "localhost:" + port + "/products/update/status"
	var wg sync.WaitGroup
	urlChan := make(chan *Data)

	go func() {
		wg.Wait()
		close(urlChan)
	}()

	for _, pid := range order.Products {
		payload := map[string]interface{}{"id": pid, "newStatus": "SOLD"}
		byts, _ := json.Marshal(payload)
		go makePOSTCall(&wg, urlChan, url, byts)
	}

	for data := range urlChan {
		if data.Err != nil {
			return "", errors.New("unable to create order.\n Product service error: " + data.Err.Error())
		}
		if data.Response.StatusCode != http.StatusOK {
			return "", errors.New("bad status code for request" + data.Response.Status)
		}
	}

	collection := repo.db.Collection("order")
	result, err := collection.InsertOne(ctx, order)
	if err != nil {
		return "", err
	}
	return result.InsertedID.(primitive.ObjectID).Hex(), err
}

// ChangeOrderStatus changes the order status
func (repo *repository) ChangeOrderStatus(ctx context.Context, orderId string, status string) error {
	collection := repo.db.Collection("order")
	opts := options.Update().SetUpsert(true)
	objID, _ := primitive.ObjectIDFromHex(orderId)
	filter := bson.D{{"_id", objID}}
	update := bson.D{{"$set", bson.D{{"status", status}}}}
	result, err := collection.UpdateOne(ctx, filter, update, opts)
	if err != nil {
		return err
	}

	if result.MatchedCount != 0 {
		level.Info(repo.logger).Log("matched and replaced an existing document")
		return nil
	}
	if result.UpsertedCount != 0 {
		level.Info(repo.logger).Log("inserted a new document with ID %v\n", result.UpsertedID)
	}
	return nil
}

// GetOrderByID query the order by given id
func (repo *repository) GetOrderByID(ctx context.Context, orderId string) (order.Order, error) {
	collection := repo.db.Collection("order")
	var result order.Order
	objID, _ := primitive.ObjectIDFromHex(orderId)
	filter := bson.D{{"_id", objID}}
	err := collection.FindOne(ctx, filter).Decode(&result)

	return result, err
}
func (repo *repository) GetOrdersByCustomerID(ctx context.Context, id string) ([]order.Order, error) {
	collection := repo.db.Collection("order")
	var result []order.Order

	filter := bson.D{{"customer_id", id}}
	cursor, err := collection.Find(ctx, filter)
	if err != nil {
		return nil, err
	}

	for cursor.Next(ctx) {
		var order order.Order
		cursor.Decode(&order)
		result = append(result, order)
	}

	return result, err
}

func makePOSTCall(wg *sync.WaitGroup, urlChan chan<- *Data, url string, body []byte) {
	defer wg.Done()

	resp, err := http.Post(url, "application/json", bytes.NewBuffer(body))
	if err != nil {
		urlChan <- &Data{Err: err}
		return
	}

	if resp.StatusCode != http.StatusOK {
		urlChan <- &Data{Err: fmt.Errorf("status code not OK for %s", url)}
		return
	}

	urlChan <- &Data{URL: url, Response: resp}
}
