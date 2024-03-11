package server

import (
	"fmt"

	"github.com/gin-gonic/gin"
	"gitlab.com/tednaaa/moner/internal/config"
	database "gitlab.com/tednaaa/moner/internal/database/sqlc"
	"gitlab.com/tednaaa/moner/internal/tokens"
)

type Server struct {
	config   config.Config
	store    database.Store
	jwtMaker *tokens.JWTMaker
	router   *gin.Engine
}

func NewServer(config config.Config, store database.Store) (*Server, error) {
	maker, err := tokens.NewJWTMaker(config.Auth.JwtSecret)
	if err != nil {
		return nil, fmt.Errorf("cannot create jwt maker: %w", err)
	}

	server := &Server{
		config:   config,
		store:    store,
		jwtMaker: maker,
	}

	server.setupRouter()

	return server, nil
}
