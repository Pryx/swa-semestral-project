package order

import (
	"context"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

// Order represents an order

// swagger:model
type Order struct {

	// the id of the this order
	//
	// required: false
	ID primitive.ObjectID `json:"_id,omitempty" bson:"_id,omitempty"`

	// the id of the customer who make this order
	//
	// required: true
	CustomerID string `json:"customer_id" bson:"customer_id,omitempty"`

	// the order status
	//
	// required: false
	Status string `json:"status" bson:"status,omitempty"`

	// the order creation date
	//
	// required: false
	CreatedOn int64 `json:"created_on,omitempty" bson:"created_on,omitempty"`

	// the order address
	//
	// required: true
	Address string `json:"address" bson:"address,omitempty"`

	// the list of product IDs
	//
	// required: true
	Products []string `json:"products,omitempty" bson:"products,omitempty"`
}

// Repository describes the persistence on order model
type Repository interface {
	CreateOrder(ctx context.Context, order Order) (string, error)
	GetOrderByID(ctx context.Context, id string) (Order, error)
	GetOrdersByCustomerID(ctx context.Context, id string) ([]Order, error)
	ChangeOrderStatus(ctx context.Context, id string, status string) error
}
