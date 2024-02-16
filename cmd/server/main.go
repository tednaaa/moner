package main

import (
	"context"
	"fmt"

	"github.com/uptrace/bun"
	"gitlab.com/tednaaa/moner/internal/migrations"
	"gitlab.com/tednaaa/moner/internal/router"
	"gitlab.com/tednaaa/moner/internal/storage"
	"gitlab.com/tednaaa/moner/internal/utils"
)

func Migrate(database *bun.DB) {
	fmt.Println("Migrating database...")
}

func main() {
	err := utils.LoadConfig(".")
	if err != nil {
		fmt.Println(err)
	}

	database := storage.ConnectAndGetDatabase()
	migrations.Migrate(database, context.Background())

	defer database.Close()

	router := router.SetupRouter()

	router.Run(":" + utils.Config.Port)
}
