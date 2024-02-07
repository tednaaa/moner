package storage

import (
	"database/sql"

	"github.com/uptrace/bun"
	"github.com/uptrace/bun/dialect/pgdialect"
	"github.com/uptrace/bun/driver/pgdriver"
)

var POSTGRES_DSN = "postgres://postgres:root@localhost:5432/postgres?sslmode=disable"

func ConnectAndGetDatabase() *bun.DB {
	storage := getStorage()
	database := bun.NewDB(storage, pgdialect.New())

	return database
}

func getStorage() *sql.DB {
	storage := sql.OpenDB(pgdriver.NewConnector(pgdriver.WithDSN(POSTGRES_DSN)))

	return storage
}
