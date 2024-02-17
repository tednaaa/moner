package user

import (
	"github.com/gin-gonic/gin"
)

func Router(router *gin.RouterGroup) {
	router.GET("", getUserRoute)
	router.POST("", createUserRoute)
	router.PUT("", updateUserRoute)
	router.DELETE("", deleteUserRoute)
}

func getUserRoute(c *gin.Context) {
	c.JSON(200, gin.H{
		"message": "Hello World!",
	})
}
func createUserRoute(c *gin.Context) {}
func updateUserRoute(c *gin.Context) {}
func deleteUserRoute(c *gin.Context) {}
