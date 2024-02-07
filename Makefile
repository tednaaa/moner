build:
	@go build -o bin/server ./cmd/server/main.go

run: build
	@./bin/server

test:
	@go test -v ./...

postgres:
	docker run -p 5432:5432 --rm --name postgres -e POSTGRES_PASSWORD=root -d postgres
