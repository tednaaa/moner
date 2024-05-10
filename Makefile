DOCKER_COMPOSE_PATH = infra/docker-compose.yml

watch:
	@cargo watch -q -c -w src/ -x run

lint:
	@cargo clippy

build:
	@cargo build

migrate-up:
	@sqlx migrate run

migrate-down:
	@sqlx migrate revert

infra-up:
	@docker compose -f $(DOCKER_COMPOSE_PATH)  up -d --remove-orphans

infra-down:
	@docker compose -f $(DOCKER_COMPOSE_PATH) down

infra-destroy:
	@docker compose -f $(DOCKER_COMPOSE_PATH) down -v

.PHONY: watch lint migrate-up migrate-down infra-up infra-down
