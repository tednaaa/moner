package api

import (
	"fmt"
	"net/http"

	"github.com/gin-gonic/gin"
)

func BadRequestError(ctx *gin.Context, err error, message string) {
	fmt.Println(err)

	ctx.JSON(http.StatusBadRequest, gin.H{"error": message})
}

func UnauthorizedError(ctx *gin.Context, err error, message string) {
	fmt.Println(err)

	ctx.JSON(http.StatusUnauthorized, gin.H{"error": message})
}

func ForbiddenError(ctx *gin.Context, err error, message string) {
	fmt.Println(err)

	ctx.JSON(http.StatusForbidden, gin.H{"error": message})
}

func NotFoundError(ctx *gin.Context, err error, message string) {
	fmt.Println(err)

	ctx.JSON(http.StatusNotFound, gin.H{"error": message})
}

func InternalServerError(ctx *gin.Context, err error, message string) {
	fmt.Println(err)

	ctx.JSON(http.StatusInternalServerError, gin.H{"error": message})
}
