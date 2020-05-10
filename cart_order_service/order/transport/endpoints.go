package transport

import (
	"context"
	"swa-semestral-project/cart_order_service/order"

	"github.com/go-kit/kit/endpoint"
)

// Endpoints holds all Go kit endpoints for the Order service.
type Endpoints struct {
	Create                endpoint.Endpoint
	GetByID               endpoint.Endpoint
	GetOrdersByCustomerID endpoint.Endpoint
	ChangeStatus          endpoint.Endpoint
	Cancel                endpoint.Endpoint
}

// MakeEndpoints initializes all Go kit endpoints for the Order service.
func MakeEndpoints(s order.Service) Endpoints {
	return Endpoints{
		Create:                makeCreateEndpoint(s),
		GetByID:               makeGetByIDEndpoint(s),
		GetOrdersByCustomerID: makeGetOrdersByCustomerIDEndpoint(s),
		ChangeStatus:          makeChangeStatusEndpoint(s),
		Cancel:                makeCancelEndpoint(s),
	}
}

func makeCreateEndpoint(s order.Service) endpoint.Endpoint {
	return func(ctx context.Context, request interface{}) (interface{}, error) {
		req := request.(CreateRequest) // type assertion
		id, err := s.Create(ctx, req.Order)
		return CreateResponse{ID: id, Err: err}, nil
	}
}

func makeGetByIDEndpoint(s order.Service) endpoint.Endpoint {
	return func(ctx context.Context, request interface{}) (interface{}, error) {
		req := request.(GetByIDRequest)
		orderRes, err := s.GetByID(ctx, req.ID)
		return GetByIDResponse{Order: orderRes, Err: err}, nil
	}
}

func makeGetOrdersByCustomerIDEndpoint(s order.Service) endpoint.Endpoint {
	return func(ctx context.Context, request interface{}) (interface{}, error) {
		req := request.(GetByIDRequest)
		orderRes, err := s.GetOrdersByCustomerID(ctx, req.ID)
		return GetOrdersByCustomerIDResponse{Orders: orderRes, Err: err}, nil
	}
}

func makeChangeStatusEndpoint(s order.Service) endpoint.Endpoint {
	return func(ctx context.Context, request interface{}) (interface{}, error) {
		req := request.(ChangeStatusRequest)
		err := s.ChangeStatus(ctx, req.ID, req.Status)
		return ChangeStatusResponse{Err: err}, nil
	}
}

func makeCancelEndpoint(s order.Service) endpoint.Endpoint {
	return func(ctx context.Context, request interface{}) (interface{}, error) {
		req := request.(CancelRequest)
		err := s.Cancel(ctx, req.ID)
		return CancelResponse{Err: err}, nil
	}
}
