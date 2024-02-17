package main

import (
	"github.com/gin-gonic/gin"
	"gitlab.com/tednaaa/moner/internal/config"
	"gitlab.com/tednaaa/moner/internal/database"
	"gitlab.com/tednaaa/moner/internal/user"
)

func main() {
	config.Load()
	database.Setup()

	gin.SetMode(config.App.GinMode)
	router := gin.Default()

	api := router.Group("/api")

	user.Router(api.Group("/user"))

	router.Run(":" + config.App.Port)
}
