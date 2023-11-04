start:
	poetry run uvicorn backend.src.main:app --reload --host localhost --port 8000
format:
	poetry run ruff format backend
lint:
	poetry run ruff check backend
test:
	poetry run pytest

start-redis:
	docker run --rm --name starter-redis -p 6379:6379 redis:latest
start-mongo:
	docker run --rm --name starter-mongo -p 27017:27017 mongo:latest
