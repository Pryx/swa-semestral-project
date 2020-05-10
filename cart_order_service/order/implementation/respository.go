package implementation

import (
	"context"
	"github.com/go-kit/kit/log/level"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"swa-semestral-project/cart_order_service/order"

	"github.com/go-kit/kit/log"
)

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
