package main

import (
	"context"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"gitlab.com/tednaaa/moner/internal/config"
	database "gitlab.com/tednaaa/moner/internal/database/sqlc"
	"gitlab.com/tednaaa/moner/internal/user"
)

func main() {
	config.Load()
	conn, queries := database.NewConnection()
	defer conn.Close(context.Background())

	gin.SetMode(config.App.GinMode)
	router := gin.Default()

	router.Use(cors.New(cors.Config{
		AllowOrigins:     []string{"http://localhost:5173"},
		AllowHeaders:     []string{"Cookie", "Content-Length", "Content-Type"},
		AllowCredentials: true,
	}))

	api := router.Group("/api")
	user.NewUserHandler(api, queries)

	router.Run(":" + config.App.Port)
}
