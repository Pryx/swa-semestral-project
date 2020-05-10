Order HTTP REST API ENDPOINTS [./order/transport/http/service.go](./order/transport/http/service.go)

Order REST API FORMAT [./order/tarnsport/request_response.go](./order/transport/request_response.go)
 
 Order DAO structure description [./order/order.go](./order/order.go)
 
 Order Service interface [./order/service.go](./order/service.go)
 
 TODO cart service
 
 
 #### How to run
 `docker-compose up --build`
 
 service api is exposed to port :8088
 
 
 TODOs:
 - tests
 - (???) fully implement card service 
 - API doc (swagger)
 - (???) elastic search
 - new field 'price' in Order (get data from another service - make http call for each product in order)