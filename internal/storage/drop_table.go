package storage

import (
	"context"
)

func ClearTable(table string, model interface{}) {
	ctx := context.Background()

	database := ConnectAndGetDatabase()
	database.NewDropTable().Table(table).Exec(ctx)

	if _, err := database.NewCreateTable().Model(model).Exec(ctx); err != nil {
		panic(err)
	}

	database.Close()
}
