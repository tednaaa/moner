package router

import (
	"github.com/gin-gonic/gin"
	"gitlab.com/tednaaa/moner/internal/user"
)

func SetupRouter() *gin.Engine {
	router := gin.Default()

	api := router.Group("/api")

	user.Router(api.Group("/user"))

	return router
}
