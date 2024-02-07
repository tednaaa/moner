package migrations

import (
	"context"
	"fmt"

	"github.com/tednaaa/moner/internal/user"
	"github.com/uptrace/bun"
)

func Migrate(database *bun.DB, ctx context.Context) {
	fmt.Println("Running migrations...")
	database.NewCreateTable().Model((*user.User)(nil)).IfNotExists().Exec(ctx)
}
