package server

import (
	"errors"
	"fmt"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/jackc/pgx/v5"
	"github.com/markbates/goth"
	"github.com/markbates/goth/gothic"
	"github.com/markbates/goth/providers/github"
	"github.com/markbates/goth/providers/gitlab"
	"github.com/markbates/goth/providers/google"
	database "gitlab.com/tednaaa/moner/internal/database/sqlc"
)

func (s *Server) setupOAuthRouter(router *gin.RouterGroup) {
	goth.UseProviders(
		google.New(s.config.Auth.GoogleClientID, s.config.Auth.GoogleClientSecret,
			oauthCallbackLink(s.config.ServerURL, "google"), "https://www.googleapis.com/auth/userinfo.email"),
		gitlab.New(s.config.Auth.GitlabClientID, s.config.Auth.GitlabClientSecret,
			oauthCallbackLink(s.config.ServerURL, "gitlab"), "read_user"),
		github.New(s.config.Auth.GithubClientID, s.config.Auth.GithubClientSecret,
			oauthCallbackLink(s.config.ServerURL, "github")),
	)

	oauthGroup := router.Group("/oauth")

	oauthGroup.GET("/:provider/callback", s.oauthCallback)
	oauthGroup.GET("/:provider", s.oauthGet)
}

func (s *Server) oauthCallback(ctx *gin.Context) {
	parseProviderParam(ctx)

	user, err := gothic.CompleteUserAuth(ctx.Writer, ctx.Request)
	if err != nil {
		InternalServerError(ctx, err, "internal error")
		return
	}

	existingUser, err := s.store.GetUserByEmail(ctx, user.Email)

	if errors.Is(err, pgx.ErrNoRows) {
		_, err = s.store.CreateUser(ctx, database.CreateUserParams{
			Email:    user.Email,
			Username: user.NickName,
		})
		if err != nil {
			InternalServerError(ctx, err, "failed to create user")
			return
		}
	} else if err != nil {
		InternalServerError(ctx, err, "internal error")
		return
	}

	generateJwtToken(ctx, *s.jwtMaker, existingUser.Email, existingUser.Username)
	ctx.Redirect(http.StatusFound, s.config.WebURL)
}

func (s *Server) oauthGet(ctx *gin.Context) {
	parseProviderParam(ctx)
	gothic.BeginAuthHandler(ctx.Writer, ctx.Request)
}

func oauthCallbackLink(apiURL string, provider string) string {
	return fmt.Sprint(apiURL, "/oauth/", provider, "/callback")
}

func parseProviderParam(ctx *gin.Context) {
	q := ctx.Request.URL.Query()
	q.Add("provider", ctx.Param("provider"))
	ctx.Request.URL.RawQuery = q.Encode()
}
