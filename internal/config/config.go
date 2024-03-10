package config

import (
	"log"

	"github.com/ilyakaznacheev/cleanenv"
	"github.com/joho/godotenv"
)

type Database struct {
	DatabaseUser     string `env:"POSTGRES_USER" env-required:"true"`
	DatabasePassword string `env:"POSTGRES_PASSWORD" env-required:"true"`
	DatabaseHost     string `env:"POSTGRES_HOST" env-required:"true"`
	DatabasePort     string `env:"POSTGRES_PORT" env-required:"true"`
	DatabaseName     string `env:"POSTGRES_DB" env-required:"true"`
}

type OAuth struct {
	GoogleClientID     string `env:"GOOGLE_CLIENT_ID" env-required:"true"`
	GoogleClientSecret string `env:"GOOGLE_CLIENT_SECRET" env-required:"true"`
	GitlabClientID     string `env:"GITLAB_CLIENT_ID" env-required:"true"`
	GitlabClientSecret string `env:"GITLAB_CLIENT_SECRET" env-required:"true"`
	GithubClientID     string `env:"GITHUB_CLIENT_ID" env-required:"true"`
	GithubClientSecret string `env:"GITHUB_CLIENT_SECRET" env-required:"true"`
}

type Config struct {
	Port       string `env:"PORT" env-required:"true"`
	GinMode    string `env:"GIN_MODE" env-required:"true"`
	Jwt_Secret string `env:"JWT_SECRET" env-required:"true"`
	Api_Url    string `env:"API_URL" env-required:"true"`
	Web_Url    string `env:"WEB_URL" env-required:"true"`
	OAuth
	Database
}

var App Config

func Load() {
	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	if err := cleanenv.ReadConfig(".env", &App); err != nil {
		log.Fatalf("cannot read config %s", err)
	}
}
