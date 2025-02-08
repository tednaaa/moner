DOCKER_COMPOSE_PATH = infra/docker-compose.yml
SERVER_PATH = apps/server

dev:
	cd $(SERVER_PATH) && \
	bacon

lint:
	cd $(SERVER_PATH) && \
	cargo clippy

build:
	cd $(SERVER_PATH) && \
	cargo build

migrate-up:
	cd $(SERVER_PATH) && \
	sqlx migrate run

migrate-down:
	cd $(SERVER_PATH) && \
	sqlx migrate revert

infra-up:
	docker compose -f $(DOCKER_COMPOSE_PATH)  up -d --remove-orphans

infra-down:
	docker compose -f $(DOCKER_COMPOSE_PATH) down

infra-destroy:
	docker compose -f $(DOCKER_COMPOSE_PATH) down -v
