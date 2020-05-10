package main

import (
	"context"
	"fmt"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.mongodb.org/mongo-driver/mongo/readpref"
	"net/http"
	"os"
	"os/signal"
	"swa-semestral-project/cart_order_service/order"
	ordersvc "swa-semestral-project/cart_order_service/order/implementation"
	"swa-semestral-project/cart_order_service/order/middleware"
	"swa-semestral-project/cart_order_service/order/transport"
	httptransport "swa-semestral-project/cart_order_service/order/transport/http"
	"syscall"
	"time"

	"github.com/go-kit/kit/log"
	"github.com/go-kit/kit/log/level"
	kitoc "github.com/go-kit/kit/tracing/opencensus"
	kithttp "github.com/go-kit/kit/transport/http"
)

func main() {
	var (
		httpAddr = ":" + getEnv("SERVER_PORT", "8080")
		dbUri    = getEnv("DB_HOST", "mongodb://root:rootpassword@localhost:27017")
		dbName   = getEnv("DB_NAME", "orders")
	)

	var logger log.Logger
	{
		logger = log.NewLogfmtLogger(os.Stderr)
		logger = log.NewSyncLogger(logger)
		logger = level.NewFilter(logger, level.AllowDebug())
		logger = log.With(logger,
			"svc", "order",
			"ts", log.DefaultTimestampUTC,
			"caller", log.DefaultCaller,
		)
	}

	level.Info(logger).Log("msg", "service started")
	defer level.Info(logger).Log("msg", "service ended")

	// Database config
	var db *mongo.Database
	{
		level.Info(logger).Log("msg", "Connecting to database", "dbUri", dbUri)
		// Database Config
		clientOpts := options.Client().ApplyURI(dbUri)
		ctx, _ := context.WithTimeout(context.Background(), 10*time.Second)

		client, err := mongo.Connect(ctx, clientOpts)
		if err != nil {
			level.Error(logger).Log("msg", "Couldn't connect to the database", err)
			os.Exit(-1)
		} //To close the connection at the end
		defer client.Disconnect(ctx)

		// Ping our db connection
		err = client.Ping(context.Background(), readpref.Primary())
		if err != nil {
			level.Error(logger).Log("msg", "Couldn't connect to the database", err)
			os.Exit(-1)
		} else {
			level.Info(logger).Log("msg", "Connected to database!")
		}
		// Create the database
		db = client.Database(dbName)
	}

	// Create Order Service
	var svc order.Service
	{
		repository, err := ordersvc.NewRepository(db, logger)
		if err != nil {
			level.Error(logger).Log("exit", err)
			os.Exit(-1)
		}
		svc = ordersvc.NewService(repository, logger)
		// Add service middleware here
		// Logging middleware
		svc = middleware.LoggingMiddleware(logger)(svc)
	}
	// Create Go kit endpoints for the Order Service
	// Then decorates with endpoint middlewares
	var endpoints transport.Endpoints
	{
		endpoints = transport.MakeEndpoints(svc)
		// Add endpoint level middlewares here
	}
	var h http.Handler
	{
		ocTracing := kitoc.HTTPServerTrace()
		serverOptions := []kithttp.ServerOption{ocTracing}
		h = httptransport.NewService(endpoints, serverOptions, logger)
	}

	errs := make(chan error)
	go func() {
		c := make(chan os.Signal)
		signal.Notify(c, syscall.SIGINT, syscall.SIGTERM)
		errs <- fmt.Errorf("%s", <-c)
	}()

	go func() {
		level.Info(logger).Log("msg", "Starting HTTP server")
		server := &http.Server{
			Addr:    httpAddr,
			Handler: h,
		}
		level.Info(logger).Log("msg", "Server up and listening on", httpAddr)
		errs <- server.ListenAndServe()
	}()

	level.Error(logger).Log("exit", <-errs)
}

func getEnv(key, fallback string) string {
	value, exists := os.LookupEnv(key)
	if !exists {
		value = fallback
	}
	return value
}
