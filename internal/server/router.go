package server

import (
	"fmt"
	"os"
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

func (s *Server) Start(address string) error {
	err := s.router.Run(address)
	if err != nil {
		return fmt.Errorf("failed to start server: %w", err)
	}
	return nil
}

func (s *Server) setupRouter() {
	router := gin.New()
	zerolog.TimestampFunc = func() time.Time { return time.Now().UTC() }
	log.Logger = log.With().Timestamp().Logger().Output(zerolog.ConsoleWriter{Out: os.Stderr})

	router.Use(RequestLogger(log.Logger))
	router.Use(cors.New(cors.Config{
		AllowOrigins:     []string{s.config.WebURL},
		AllowHeaders:     []string{"Cookie", "Content-Length", "Content-Type"},
		AllowCredentials: true,
	}))

	router.ForwardedByClientIP = true
	err := router.SetTrustedProxies([]string{"127.0.0.1"})
	if err != nil {
		log.Fatal().Err(err).Msg("cannot set trusted proxies")
	}

	api := router.Group("/api")

	s.setupUserRouter(api)
	s.setupOAuthRouter(api)

	s.router = router
}
