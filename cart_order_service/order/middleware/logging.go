package middleware

import (
	"context"
	"github.com/go-kit/kit/log/level"
	"swa-semestral-project/cart_order_service/order"
	"time"

	"github.com/go-kit/kit/log"
)

func LoggingMiddleware(logger log.Logger) Middleware {
	return func(next order.Service) order.Service {
		return &loggingMiddleware{
			next:   next,
			logger: level.Debug(logger),
		}
	}
}

type loggingMiddleware struct {
	next   order.Service
	logger log.Logger
}

func (mw loggingMiddleware) Create(ctx context.Context, order order.Order) (id string, err error) {
	defer func(begin time.Time) {
		mw.logger.Log("method", "Create", "CustomerID", order.CustomerID, "took", time.Since(begin), "err", err)
	}(time.Now())
	return mw.next.Create(ctx, order)
}

func (mw loggingMiddleware) GetByID(ctx context.Context, id string) (order order.Order, err error) {
	defer func(begin time.Time) {
		mw.logger.Log("method", "GetByID", "OrderID", id, "took", time.Since(begin), "err", err)
	}(time.Now())
	return mw.next.GetByID(ctx, id)
}

func (mw loggingMiddleware) GetOrdersByCustomerID(ctx context.Context, id string) (order []order.Order, err error) {
	defer func(begin time.Time) {
		mw.logger.Log("method", "GetOrdersByCustomerID", "OrderID", id, "took", time.Since(begin), "err", err)
	}(time.Now())
	return mw.next.GetOrdersByCustomerID(ctx, id)
}

func (mw loggingMiddleware) ChangeStatus(ctx context.Context, id string, status string) (err error) {
	defer func(begin time.Time) {
		mw.logger.Log("method", "ChangeStatus", "OrderID", id, "took", time.Since(begin), "err", err)
	}(time.Now())
	return mw.next.ChangeStatus(ctx, id, status)
}

func (mw loggingMiddleware) Cancel(ctx context.Context, id string) (err error) {
	defer func(begin time.Time) {
		mw.logger.Log("method", "Cancel", "OrderID", id, "took", time.Since(begin), "err", err)
	}(time.Now())
	return mw.next.Cancel(ctx, id)
}
