package user

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func Router(router *gin.RouterGroup) {
	router.GET("", getUserRoute)
	router.POST("", createUserRoute)
	router.PUT("", updateUserRoute)
	router.DELETE("", deleteUserRoute)
}

func getUserRoute(c *gin.Context) {}

func createUserRoute(c *gin.Context) {
	var user CreateUserDto

	if err := c.ShouldBindJSON(&user); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	if err := createUser(&user); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusCreated, gin.H{})
}

func updateUserRoute(c *gin.Context) {}
func deleteUserRoute(c *gin.Context) {}
