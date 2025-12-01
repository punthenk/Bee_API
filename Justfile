# Set default recipe to list all commands
default:
    @just --list

# Start the database
db-start:
    docker-compose up -d
    @echo "Waiting for database to be started..."
    @sleep 5

# Stop the database
db-stop:
    docker-compose down

# Reset database
db-reset:
    docker-compose down -v
    docker-compose up -d db
    @sleep 5
    @echo "Database reset done"

# Run migrations
migrate:
    sqlx migrate run

# Create a new migration
migrate-create name:
    sqlx migrate add {{name}}

# Revert last migration
migrate-revert:
    sqlx migrate revert

# Seed the database with test data
seed:
    @echo "Seeding database..."
    cargo run --bin seed

# Complete setup for new team members
setup:
    @echo "Setting up Beekeeper API..."
    docker-compose up -d
    @sleep 5
    sqlx migrate run
    cargo run --bin seed
    @echo "Setup complete"
