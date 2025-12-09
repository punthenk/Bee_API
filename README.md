# Beekeeper API

A Rust-based API for managing beehive data, including hives, queens, inspections, and user management.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- [Rust](https://rustup.rs/) (latest stable version)
- [Just](https://github.com/casey/just) command runner

	Install Just:

```bash
# macOS
brew install just

# Linux
cargo install just

# Windows
cargo install just
```

## Quick Start

1. **Clone the repository**

    ```bash
	   git clone <repository-url>
	   cd beekeeper-api
   ```

2. **Start the Docker containers**

    ```bash
    docker-compose up -d
    # or
    just db-start
    # or if you want to run the api in a container as well
    docker-compose down --profile production
    ```
    
    This starts the MySQL database container in the background.
    
3. **Build the database schema**
    
    ```bash
    just db-reset
    ```
    
    This creates all the necessary tables (users, queen, hives, inspections).
    
4. **Seed with test data**
    
    ```bash
    just seed
    ```
    
    This populates the database with fake data for testing.
    
5. **Run the API**
    
    ```bash
    just run
    # or
    cargo run
    ```
    
    The API will be available at `http://localhost:3000`
    

## Database Management

### When Someone Updates the Database Schema

If a team member changes the database structure in `schema.sql`:

1. **Pull the latest changes**
    
    ```bash
    git pull
    ```
    
2. **Reset your database to the new schema**
    
    ```bash
    just db-reset
    ```
    
    This will drop all existing tables and recreate them with the new structure.
    
3. **Reseed the database**
    
    ```bash
    just seed
    ```
    
    This adds fresh test data to work with.
    

That's it! Your database is now up to date with the latest schema.

## Available Commands

View all available commands:

```bash
just
```

### Common Commands

| Command         | Description                           |
| --------------- | ------------------------------------- |
| `just run`      | Start the API server                  |
| `just db-start` | Start the database container          |
| `just db-reset` | Drop and recreate all database tables |
| `just seed`     | Populate database with test data      |

### Port already in use

If port 3000 or 3306 is already in use, you can either:

1. Stop the application using that port
2. Change the port mapping in `docker-compose.yml`

### Fresh start

For a complete fresh start:

```bash
docker-compose down -v  # Stop containers and remove volumes
docker-compose up -d    # Start fresh containers
just db-reset           # Create schema
just seed               # Add test data
```

## Contributing

When making changes to the database schema:

1. Update `schema.sql` with your changes
2. Test locally with `just db-reset`
3. Commit both code and schema changes
4. Notify team members to reset the after pulling
