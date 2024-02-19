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

type Config struct {
	Port    string `env:"PORT" env-required:"true"`
	GinMode string `env:"GIN_MODE" env-required:"true"`
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
