package middleware

import "swa-semestral-project/cart_order_service/order"

// Middleware describes a service middleware.
type Middleware func(service order.Service) order.Service
