# Set default recipe to list all commands
default:
    @just --list

run:
    clear && cargo run

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
    @echo "Dropping tables..."
    docker exec -i beekeeper_db mysql -uadmin -proot beekeeper_db < reset.sql
    @echo "Recreating schema..."
    docker exec -i beekeeper_db mysql -uadmin -proot beekeeper_db < schema.sql
    @echo "Database reset done"

# Seed the database with test data
seed:
    @echo "Seeding database..."
    cargo run --bin seed
