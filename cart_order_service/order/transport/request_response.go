package transport

import "swa-semestral-project/cart_order_service/order"

// CreateRequest holds the request parameters for the Create method.

// swagger:parameters createOrder
type CreateRequest struct {
	Order order.Order
}

// CreateResponse holds the response values for the Create method.
// swagger:response createResponse
type CreateResponse struct {
	// An optional field with I of the created order
	ID string `json:"id,omitempty"`
	// An optional field indicating and error
	Err error `json:"error,omitempty"`
}

// Failed implements endpoint.Failer.
func (r CreateResponse) Failed() error { return r.Err }

// GetByIDRequest holds the request parameters for the GetByID method.
type GetByIDRequest struct {
	ID string
}

// GetByIDResponse holds the response values for the GetByID method.

// swagger:response getByIDResponse
type GetByIDResponse struct {
	// An optional field with requested order
	Order order.Order `json:"order,omitempty"`
	// An optional field indicating and error
	Err error `json:"error,omitempty"`
}

// Failed implements endpoint.Failer.
func (r GetByIDResponse) Failed() error { return r.Err }

// GetOrdersByCustomerIDResponse  holds the response values for the GetOrdersByCustomerID method.

// swagger:response getOrdersByCustomerIDResponse
type GetOrdersByCustomerIDResponse struct {
	// An optional field with all orders
	Orders []order.Order `json:"orders,omitempty"`
	// An optional field indicating and error
	Err error `json:"error,omitempty"`
}

// Failed implements endpoint.Failer.
func (r GetOrdersByCustomerIDResponse) Failed() error { return r.Err }

// ChangeStatusRequest holds the request parameters for the ChangeStatus method.

// swagger:parameters changeOrderStatus
type ChangeStatusRequest struct {
	ID     string `json:"id"`
	Status string `json:"status"`
}

// ChangeStatusResponse holds the response values for the ChangeStatus method.
// swagger:response changeStatusResponse
type ChangeStatusResponse struct {
	// An optional field indicating and error
	Err error `json:"error,omitempty"`
}

// Failed implements endpoint.Failer.
func (r ChangeStatusResponse) Failed() error { return r.Err }

type CancelRequest struct {
	ID string
}

// swagger:response cancelResponse
type CancelResponse struct {
	// An optional field indicating and error
	Err error `json:"error,omitempty"`
}

// Failed implements endpoint.Failer.
func (r CancelResponse) Failed() error { return r.Err }
