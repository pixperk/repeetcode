# Set your environment .env file
ENV_FILE = .env
DB_URL = $(shell grep DATABASE_URL $(ENV_FILE) | cut -d '=' -f2-)

# Commands
create-db:
	sqlx database create

drop-db:
	sqlx database drop --yes

migrate:
	sqlx migrate run

new-migration:
	@if [ -z "$(name)" ]; then \
		echo "❌ Missing migration name. Usage: make new-migration name=your_migration"; \
		exit 1; \
	fi; \
	sqlx migrate add $(name)

redo-migration:
	sqlx migrate revert && sqlx migrate run

reset-db:
	sqlx migrate revert --all && sqlx migrate run

status:
	sqlx migrate info

.PHONY: create-db drop-db migrate new-migration redo-migration reset-db status
