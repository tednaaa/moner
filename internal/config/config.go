package config

import (
	"log"

	"github.com/ilyakaznacheev/cleanenv"
	"github.com/joho/godotenv"
)

type app struct {
	Port    string `env:"PORT" env-required:"true"`
	GinMode string `env:"GIN_MODE" env-required:"true"`
	Api_Url string `env:"API_URL" env-required:"true"`
	Web_Url string `env:"WEB_URL" env-required:"true"`
}

type database struct {
	User     string `env:"POSTGRES_USER" env-required:"true"`
	Password string `env:"POSTGRES_PASSWORD" env-required:"true"`
	Host     string `env:"POSTGRES_HOST" env-required:"true"`
	Port     string `env:"POSTGRES_PORT" env-required:"true"`
	Name     string `env:"POSTGRES_DB" env-required:"true"`
}

type auth struct {
	Jwt_Secret         string `env:"JWT_SECRET" env-required:"true"`
	GoogleClientID     string `env:"GOOGLE_CLIENT_ID" env-required:"true"`
	GoogleClientSecret string `env:"GOOGLE_CLIENT_SECRET" env-required:"true"`
	GitlabClientID     string `env:"GITLAB_CLIENT_ID" env-required:"true"`
	GitlabClientSecret string `env:"GITLAB_CLIENT_SECRET" env-required:"true"`
	GithubClientID     string `env:"GITHUB_CLIENT_ID" env-required:"true"`
	GithubClientSecret string `env:"GITHUB_CLIENT_SECRET" env-required:"true"`
}

type email struct {
	SenderName     string `env:"EMAIL_SENDER_NAME" env-required:"true"`
	SenderAddress  string `env:"EMAIL_SENDER_ADDRESS" env-required:"true"`
	SenderPassword string `env:"EMAIL_SENDER_PASSWORD" env-required:"true"`
}

var App app
var Database database
var Auth auth
var Email email
var configs = []interface{}{
	&App,
	&Database,
	&Auth,
	&Email,
}

func Load(path string) {
	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	for _, config := range configs {
		if err := cleanenv.ReadConfig(path, config); err != nil {
			log.Fatalf("cannot read config for %T: %s", config, err)
		}
	}
}
