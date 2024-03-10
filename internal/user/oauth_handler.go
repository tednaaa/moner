package user

import (
	"fmt"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/jackc/pgx/v5"
	"github.com/markbates/goth"
	"github.com/markbates/goth/gothic"
	"github.com/markbates/goth/providers/github"
	"github.com/markbates/goth/providers/gitlab"
	"github.com/markbates/goth/providers/google"
	"gitlab.com/tednaaa/moner/internal/api"
	"gitlab.com/tednaaa/moner/internal/config"
	database "gitlab.com/tednaaa/moner/internal/database/sqlc"
)

type oauthHandler struct {
	queries database.Queries
}

func NewOAuthHandler(api *gin.RouterGroup, queries database.Queries) {
	oauthHandler := &oauthHandler{queries: queries}

	goth.UseProviders(
		google.New(config.App.GoogleClientID, config.App.GoogleClientSecret, oAuthCallbackLink("google"), "https://www.googleapis.com/auth/userinfo.email"),
		gitlab.New(config.App.GitlabClientID, config.App.GitlabClientSecret, oAuthCallbackLink("gitlab"), "read_user"),
		github.New(config.App.GithubClientID, config.App.GithubClientSecret, oAuthCallbackLink("github")),
	)

	oauthGroup := api.Group("/oauth")

	oauthGroup.GET("/:provider/callback", oauthHandler.callback)
	oauthGroup.GET("/:provider", oauthHandler.get)
}

func oAuthCallbackLink(provider string) string {
	return fmt.Sprint(config.App.Api_Url, "/oauth/", provider, "/callback")
}

func parseProviderParam(ctx *gin.Context) {
	q := ctx.Request.URL.Query()
	q.Add("provider", ctx.Param("provider"))
	ctx.Request.URL.RawQuery = q.Encode()
}

func (o *oauthHandler) callback(ctx *gin.Context) {
	parseProviderParam(ctx)

	user, err := gothic.CompleteUserAuth(ctx.Writer, ctx.Request)
	if err != nil {
		api.InternalServerError(ctx, err, "internal error")
		return
	}

	existingUser, err := o.queries.GetUserByEmail(ctx, user.Email)
	if err == pgx.ErrNoRows {
		_, err = o.queries.CreateUser(ctx, database.CreateUserParams{
			Email:    user.Email,
			Username: user.NickName,
		})
		if err != nil {
			api.InternalServerError(ctx, err, "internal error")
			return
		}
	} else if err != nil {
		api.InternalServerError(ctx, err, "internal error")
		return
	}

	fmt.Println(user, "user from oauth>>>>>>>>>>")
	fmt.Println(existingUser, "existingUser>>>>>>>>>>")

	generateJwtToken(ctx, existingUser.Email, existingUser.Username)
	ctx.Redirect(http.StatusFound, config.App.Web_Url)
}

func (o *oauthHandler) get(ctx *gin.Context) {
	parseProviderParam(ctx)
	gothic.BeginAuthHandler(ctx.Writer, ctx.Request)
}
