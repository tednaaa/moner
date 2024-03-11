package server

import (
	"fmt"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/rs/zerolog/log"
)

func BadRequestError(ctx *gin.Context, err error, message string) {
	log.Error().Err(err).Msg(fmt.Sprintf("bad request (%s)", message))

	ctx.JSON(http.StatusBadRequest, gin.H{"error": message})
}

func UnauthorizedError(ctx *gin.Context, err error, message string) {
	log.Error().Err(err).Msg(fmt.Sprintf("unauthorized (%s)", message))

	ctx.JSON(http.StatusUnauthorized, gin.H{"error": message})
}

func ForbiddenError(ctx *gin.Context, err error, message string) {
	log.Error().Err(err).Msg(fmt.Sprintf("forbidden (%s)", message))

	ctx.JSON(http.StatusForbidden, gin.H{"error": message})
}

func NotFoundError(ctx *gin.Context, err error, message string) {
	log.Error().Err(err).Msg(fmt.Sprintf("not found (%s)", message))

	ctx.JSON(http.StatusNotFound, gin.H{"error": message})
}

func InternalServerError(ctx *gin.Context, err error, message string) {
	log.Error().Err(err).Msg(fmt.Sprintf("internal (%s)", message))

	ctx.JSON(http.StatusInternalServerError, gin.H{"error": message})
}
