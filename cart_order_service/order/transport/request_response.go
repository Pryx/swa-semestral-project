package transport

import "swa-semestral-project/cart_order_service/order"

// CreateRequest holds the request parameters for the Create method.

// swagger:parameters createOrder
type CreateRequest struct {
	// CreateRequest holds the request parameters for the Create method.
	//
	//in:body
	Order order.Order
}

// swagger:response createResponse
type CreateResponse struct {
	// CreateResponse holds the response values for the Create method.
	//
	// An optional field with I of the created order
	//in:body
	ID string `json:"id,omitempty"`
	//in:body
	// An optional field indicating and error
	Err error `json:"error,omitempty"`
}

// Failed implements endpoint.Failer.
func (r CreateResponse) Failed() error { return r.Err }

type GetByIDRequest struct {
	// GetByIDRequest holds the request parameters for the GetByID method.
	//
	ID string
}

// swagger:response getByIDResponse
type GetByIDResponse struct {
	// GetByIDResponse holds the response values for the GetByID method.
	//
	// An optional field with requested order
	Order order.Order `json:"order,omitempty"`
	// An optional field indicating and error
	Err error `json:"error,omitempty"`
}

// Failed implements endpoint.Failer.
func (r GetByIDResponse) Failed() error { return r.Err }

// swagger:response getOrdersByCustomerIDResponse
type GetOrdersByCustomerIDResponse struct {
	// GetOrdersByCustomerIDResponse  holds the response values for the GetOrdersByCustomerID method.
	//
	// An optional field with all orders
	Orders []order.Order `json:"orders,omitempty"`
	// An optional field indicating and error
	Err error `json:"error,omitempty"`
}

// Failed implements endpoint.Failer.
func (r GetOrdersByCustomerIDResponse) Failed() error { return r.Err }

// swagger:parameters changeOrderStatus
type ChangeStatusRequest struct {
	// ChangeStatusRequest holds the request parameters for the ChangeStatus method.
	//
	//in:body
	ID string `json:"id"`
	//in:body
	Status string `json:"status"`
}

// swagger:response changeStatusResponse
type ChangeStatusResponse struct {
	// ChangeStatusResponse holds the response values for the ChangeStatus method.
	//
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
	// CancelResponse holds the response values for the CancelResponse method.
	//
	// An optional field indicating and error
	Err error `json:"error,omitempty"`
}

// Failed implements endpoint.Failer.
func (r CancelResponse) Failed() error { return r.Err }
