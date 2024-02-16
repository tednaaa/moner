package migrations

import (
	"context"
	"fmt"

	"github.com/uptrace/bun"
	"gitlab.com/tednaaa/moner/internal/user"
)

func Migrate(database *bun.DB, ctx context.Context) {
	fmt.Println("Running migrations...")
	database.NewCreateTable().Model((*user.User)(nil)).IfNotExists().Exec(ctx)
}
