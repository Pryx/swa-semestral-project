package order

import (
	"context"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

// Order represents an order
type Order struct {
	ID         primitive.ObjectID `json:"_id,omitempty" bson:"_id,omitempty"`
	CustomerID string             `json:"customer_id" bson:"customer_id,omitempty"`
	Status     string             `json:"status" bson:"status,omitempty"`
	CreatedOn  int64              `json:"created_on,omitempty" bson:"created_on,omitempty"`
	Address    string             `json:"address" bson:"address,omitempty"`
	Products   []string           `json:"products,omitempty" bson:"products,omitempty"`
}

// Repository describes the persistence on order model
type Repository interface {
	CreateOrder(ctx context.Context, order Order) (string, error)
	GetOrderByID(ctx context.Context, id string) (Order, error)
	GetOrdersByCustomerID(ctx context.Context, id string) ([]Order, error)
	ChangeOrderStatus(ctx context.Context, id string, status string) error
}
