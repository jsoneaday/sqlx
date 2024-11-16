docker rm -f sqlx-db
docker build -t sqlx-i .
docker run --name sqlx-db -d -p 5433:5432 sqlx-i

sqlx database create --database-url postgres://sqlx:sqlx@localhost:5433/sqlx
sqlx migrate run --database-url postgres://sqlx:sqlx@localhost:5433/sqlx