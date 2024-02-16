build:
	@go build -o bin/server ./cmd/server/main.go

run: build
	@./bin/server

test:
	@go test -v ./...

postgres:
	docker run --rm -d \
		--name temp_db \
		-e POSTGRES_USER=root \
		-e POSTGRES_PASSWORD=root \
		-e POSTGRES_DB=moner \
		-p 5432:5432 \
		postgres:16.2
