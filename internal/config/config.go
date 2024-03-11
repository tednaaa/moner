package config

import (
	"log"

	"github.com/ilyakaznacheev/cleanenv"
	"github.com/joho/godotenv"
)

type database struct {
	User     string `env:"POSTGRES_USER" env-required:"true"`
	Password string `env:"POSTGRES_PASSWORD" env-required:"true"`
	Host     string `env:"POSTGRES_HOST" env-required:"true"`
	Port     string `env:"POSTGRES_PORT" env-required:"true"`
	Name     string `env:"POSTGRES_DB" env-required:"true"`
}

func (d *database) GetPostgresDSN() string {
	return "postgres://" + d.User + ":" + d.Password + "@" + d.Host + ":" + d.Port + "/" + d.Name
}

type auth struct {
	JwtSecret          string `env:"JWT_SECRET" env-required:"true"`
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

type Config struct {
	Database  database
	Auth      auth
	Email     email
	GinMode   string `env:"GIN_MODE" env-required:"true"`
	Port      string `env:"PORT" env-required:"true"`
	ServerURL string `env:"SERVER_URL" env-required:"true"`
	WebURL    string `env:"WEB_URL" env-required:"true"`
}

func Load(path string) *Config {
	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	var config Config
	err = cleanenv.ReadConfig(path, &config)
	if err != nil {
		log.Fatalf("cannot read config for %T: %s", config, err)
	}

	return &config
}
