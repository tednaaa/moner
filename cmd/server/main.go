package main

import (
	"context"

	"github.com/gin-gonic/gin"
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"gitlab.com/tednaaa/moner/internal/config"
	database "gitlab.com/tednaaa/moner/internal/database/sqlc"
	"gitlab.com/tednaaa/moner/internal/server"
)

func main() {
	config := config.Load(".env")

	gin.SetMode(config.GinMode)
	zerolog.SetGlobalLevel(zerolog.TraceLevel)

	connPool, err := pgxpool.New(context.Background(), config.Database.GetPostgresDSN())
	if err != nil {
		log.Fatal().Err(err).Msg("cannot connect to db")
	}
	store := database.NewStore(connPool)

	server, err := server.NewServer(*config, *store)
	if err != nil {
		log.Fatal().Err(err).Msg("cannot create server")
	}

	err = server.Start(":" + config.Port)
	if err != nil {
		log.Fatal().Err(err).Msg("cannot start server")
	}
}
