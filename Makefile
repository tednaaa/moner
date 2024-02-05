build:
	@go build -o bin/moner

run: build
	@./bin/moner

test:
	@go test -v ./...
