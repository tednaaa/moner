package utils

import (
	"github.com/spf13/viper"
)

type config struct {
	Environment      string `mapstructure:"ENVIRONMENT"`
	Port             string `mapstructure:"PORT"`
	PostgresUser     string `mapstructure:"POSTGRES_USER"`
	PostgresPassword string `mapstructure:"POSTGRES_PASSWORD"`
	PostgresDB       string `mapstructure:"POSTGRES_DB"`
	PostgresHost     string `mapstructure:"POSTGRES_HOST"`
	PostgresPort     string `mapstructure:"POSTGRES_PORT"`
}

var Config config

func LoadConfig(path string) (err error) {
	viper.AddConfigPath(path)
	viper.SetConfigName("app")
	viper.SetConfigType("env")

	viper.BindEnv("ENVIRONMENT")
	viper.BindEnv("PORT")
	viper.BindEnv("POSTGRES_USER")
	viper.BindEnv("POSTGRES_PASSWORD")
	viper.BindEnv("POSTGRES_DB")
	viper.BindEnv("POSTGRES_HOST")
	viper.BindEnv("POSTGRES_PORT")

	viper.AutomaticEnv()
	viper.ReadInConfig()

	err = viper.Unmarshal(&Config)
	return
}
