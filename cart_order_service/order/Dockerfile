FROM golang:alpine as builder

RUN mkdir /app
WORKDIR /app

# Copy and download dependency using go mod
COPY . .
RUN GO111MODULE=on go mod download

# Build the application
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -a -installsuffix cgo -o order ./cmd/main.go
RUN chmod +x ./order

# Run container
FROM alpine:latest

RUN mkdir /app
WORKDIR /app
COPY --from=builder /app/order /app/

ENTRYPOINT ["./order"]
CMD ""