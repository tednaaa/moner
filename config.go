package main

import (
	"log"
	"os"

	"github.com/joho/godotenv"
)

type Config struct {
	port string
}

func setupConfg() Config {
	err := godotenv.Load()

	if err != nil {
		log.Fatal("Error loading .env file")
	}

	return Config{
		port: os.Getenv("PORT"),
	}
}
