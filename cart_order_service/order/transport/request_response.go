package transport

import "swa-semestral-project/cart_order_service/order"

// CreateRequest holds the request parameters for the Create method.
type CreateRequest struct {
	Order order.Order
}

// CreateResponse holds the response values for the Create method.
type CreateResponse struct {
	ID  string `json:"id,omitempty"`
	Err error  `json:"error,omitempty"`
}

// Failed implements endpoint.Failer.
func (r CreateResponse) Failed() error { return r.Err }

// GetByIDRequest holds the request parameters for the GetByID method.
type GetByIDRequest struct {
	ID string
}

// GetByIDResponse holds the response values for the GetByID method.
type GetByIDResponse struct {
	Order order.Order `json:"order"`
	Err   error       `json:"error,omitempty"`
}

// Failed implements endpoint.Failer.
func (r GetByIDResponse) Failed() error { return r.Err }

// GetOrdersByCustomerIDResponse  holds the response values for the GetOrdersByCustomerID method.
type GetOrdersByCustomerIDResponse struct {
	Orders []order.Order `json:"orders,omitempty"`
	Err    error         `json:"error,omitempty"`
}

// Failed implements endpoint.Failer.
func (r GetOrdersByCustomerIDResponse) Failed() error { return r.Err }

// ChangeStatusRequest holds the request parameters for the ChangeStatus method.
type ChangeStatusRequest struct {
	ID     string `json:"id"`
	Status string `json:"status"`
}

// ChangeStatusResponse holds the response values for the ChangeStatus method.
type ChangeStatusResponse struct {
	Err error `json:"error,omitempty"`
}

// Failed implements endpoint.Failer.
func (r ChangeStatusResponse) Failed() error { return r.Err }

type CancelRequest struct {
	ID string
}

type CancelResponse struct {
	Err error `json:"error,omitempty"`
}

// Failed implements endpoint.Failer.
func (r CancelResponse) Failed() error { return r.Err }
