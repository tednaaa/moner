POSTGRES_VERSION := 16.2
POSTGRES_CONTAINER_NAME := temp_db
POSTGRES_USER := root
POSTGRES_PASSWORD := root
POSTGRES_DB := moner
POSTGRES_PORT := 5432
POSTGRES_DSN := postgres://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@$(POSTGRES_HOST):$(POSTGRES_PORT)/$(POSTGRES_DB)?sslmode=disable
MIGRATIONS_PATH := internal/database/migrations

build:
	@go build -o bin/server ./cmd/server/main.go

run: build
	@./bin/server

test:
	@go test -v -cover ./...

lint:
	@golangci-lint run

postgres:
	@docker run --rm -d \
		--name $(POSTGRES_CONTAINER_NAME) \
		-e POSTGRES_USER=$(POSTGRES_USER) \
		-e POSTGRES_PASSWORD=$(POSTGRES_PASSWORD) \
		-e POSTGRES_DB=$(POSTGRES_DB) \
		-p $(POSTGRES_PORT):$(POSTGRES_PORT) \
		postgres:$(POSTGRES_VERSION)

migrate-up:
	@migrate -path $(MIGRATIONS_PATH) \
		-database $(POSTGRES_DSN) \
		-verbose up

migrate-down:
	@migrate -path $(MIGRATIONS_PATH) \
		-database $(POSTGRES_DSN) \
		-verbose down

new_migration:
	@migrate create -ext sql -dir $(MIGRATIONS_PATH) -seq $(name)

sqlc:
	@sqlc generate

.PHONY: build run test postgres migrate-up migrate-down new_migration sqlc
