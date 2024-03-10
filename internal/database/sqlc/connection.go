package database

import (
	"context"
	"fmt"
	"log"

	"github.com/jackc/pgx/v5"
	"gitlab.com/tednaaa/moner/internal/config"
)

func NewConnection() (*pgx.Conn, Queries) {
	ctx := context.Background()

	conn, err := pgx.Connect(ctx, getPostgresDSN())
	if err != nil {
		log.Fatal("Unable to connect to database:", err)
	}

	queries := New(conn)

	return conn, *queries
}

func getPostgresDSN() string {
	return fmt.Sprintf(
		"postgres://%v:%v@%v:%v/%v",
		config.Database.User, config.Database.Password,
		config.Database.Host, config.Database.Port, config.Database.Name,
	)
}
