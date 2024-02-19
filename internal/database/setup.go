package database

import (
	"context"
	"fmt"
	"log"
	"os"

	"github.com/jackc/pgx/v5"
	"gitlab.com/tednaaa/moner/internal/config"
)

func Setup() {
	connection, err := pgx.Connect(context.Background(), getPostgresDSN())

	if err != nil {
		log.Fatal("Unable to connect to database:", err)
		os.Exit(1)
	}
	defer connection.Close(context.Background())
}

func getPostgresDSN() string {
	return fmt.Sprintf(
		"postgres://%v:%v@%v:%v/%v",
		config.App.DatabaseUser, config.App.DatabasePassword,
		config.App.DatabaseHost, config.App.DatabasePort, config.App.DatabaseName,
	)
}
