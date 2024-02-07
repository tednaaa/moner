package main

import (
	"context"
	"fmt"

	"github.com/tednaaa/moner/internal/config"
	"github.com/tednaaa/moner/internal/migrations"
	"github.com/tednaaa/moner/internal/router"
	"github.com/tednaaa/moner/internal/storage"
	"github.com/uptrace/bun"
)

func Migrate(database *bun.DB) {
	fmt.Println("Migrating database...")
}

func main() {
	database := storage.ConnectAndGetDatabase()
	migrations.Migrate(database, context.Background())

	defer database.Close()

	config := config.SetupConfg()
	router := router.SetupRouter()

	router.Run(":" + config.Port)
}
