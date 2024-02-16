package storage

import (
	"database/sql"
	"fmt"

	"github.com/uptrace/bun"
	"github.com/uptrace/bun/dialect/pgdialect"
	"github.com/uptrace/bun/driver/pgdriver"
	"gitlab.com/tednaaa/moner/internal/utils"
)

func ConnectAndGetDatabase() *bun.DB {
	storage := getStorage()
	database := bun.NewDB(storage, pgdialect.New())

	return database
}

func getStorage() *sql.DB {
	var POSTGRES_DSN = fmt.Sprintf(
		"postgres://%s:%s@%s:%s/%s?sslmode=disable",
		utils.Config.PostgresUser,
		utils.Config.PostgresPassword,
		utils.Config.PostgresHost,
		utils.Config.PostgresPort,
		utils.Config.PostgresDB,
	)

	storage := sql.OpenDB(pgdriver.NewConnector(pgdriver.WithDSN(POSTGRES_DSN)))

	return storage
}
