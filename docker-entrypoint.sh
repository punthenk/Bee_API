#!/bin/bash
set -e

echo "Waiting for database to be ready..."
until mysql -h db -uadmin -proot -e "SELECT 1" &> /dev/null; do
    echo "Database is unavailable - sleeping"
    sleep 2
done

echo "Database is ready!"

echo "Resetting database schema..."
mysql -h db -uadmin -proot beekeeper_db < /app/reset.sql
mysql -h db -uadmin -proot beekeeper_db < /app/schema.sql

echo "Seeding database..."
/usr/local/bin/seed

echo "Starting API server..."
exec /usr/local/bin/default
