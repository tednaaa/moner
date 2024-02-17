package main

import (
	"fmt"

	"gitlab.com/tednaaa/moner/internal/router"
	"gitlab.com/tednaaa/moner/internal/utils"
)

func main() {
	err := utils.LoadConfig(".")
	if err != nil {
		fmt.Println(err)
	}

	router := router.SetupRouter()

	router.Run(":" + utils.Config.Port)
}
