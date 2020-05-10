package implementation

import (
	"context"
	"database/sql"
	"time"

	"github.com/go-kit/kit/log"
	"github.com/go-kit/kit/log/level"
	ordersvc "swa-semestral-project/cart_order_service/order"
)

// service implements the Order Service
type service struct {
	// TODO wire with another service e.g. delivery service
	repository ordersvc.Repository
	logger     log.Logger
}

// NewService creates and returns a new Order service instance
func NewService(rep ordersvc.Repository, logger log.Logger) ordersvc.Service {
	return &service{
		repository: rep,
		logger:     logger,
	}
}

// Create makes an order
func (s *service) Create(ctx context.Context, order ordersvc.Order) (string, error) {
	logger := log.With(s.logger, "method", "Create")
	order.Status = "Pending"
	order.CreatedOn = time.Now().Unix()

	id, err := s.repository.CreateOrder(ctx, order)
	if err != nil {
		level.Error(logger).Log("err", err)
		return "", ordersvc.ErrCmdRepository
	}
	return id, nil
}

// GetByID returns an order given by id
func (s *service) GetByID(ctx context.Context, id string) (ordersvc.Order, error) {
	logger := log.With(s.logger, "method", "GetByID")
	order, err := s.repository.GetOrderByID(ctx, id)
	if err != nil {
		level.Error(logger).Log("err", err)
		if err == sql.ErrNoRows {
			return order, ordersvc.ErrOrderNotFound
		}
		return order, ordersvc.ErrQueryRepository
	}
	return order, nil
}

// GetByID returns an order given by id
func (s *service) GetOrdersByCustomerID(ctx context.Context, id string) ([]ordersvc.Order, error) {
	logger := log.With(s.logger, "method", "GetByCustomerID")
	orders, err := s.repository.GetOrdersByCustomerID(ctx, id)
	if err != nil {
		level.Error(logger).Log("err", err)
		if err == sql.ErrNoRows {
			return orders, ordersvc.ErrOrderNotFound
		}
		return orders, ordersvc.ErrQueryRepository
	}
	return orders, nil
}

// ChangeStatus changes the status of an order
func (s *service) ChangeStatus(ctx context.Context, id string, status string) error {
	logger := log.With(s.logger, "method", "ChangeStatus")
	if err := s.repository.ChangeOrderStatus(ctx, id, status); err != nil {
		level.Error(logger).Log("err", err)
		return ordersvc.ErrCmdRepository
	}
	return nil
}

// Cancel changes the status of an order
func (s *service) Cancel(ctx context.Context, id string) error {
	logger := log.With(s.logger, "method", "Cancel")
	if err := s.repository.ChangeOrderStatus(ctx, id, "Canceled"); err != nil {
		level.Error(logger).Log("err", err)
		return ordersvc.ErrCmdRepository
	}
	return nil
}
