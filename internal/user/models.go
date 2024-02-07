package user

import (
	"context"
	"time"

	"github.com/tednaaa/moner/internal/storage"
	"github.com/uptrace/bun"
	"golang.org/x/crypto/bcrypt"
)

type User struct {
	bun.BaseModel `bun:"table:users"`

	ID           int64     `bun:"id,pk,autoincrement"`
	Email        string    `bun:"email,unique,notnull"`
	Username     string    `bun:"username,unique,notnull"`
	PasswordHash string    `bun:"password,notnull"`
	CreatedAt    time.Time `bun:"created_at,notnull"`
	UpdatedAt    time.Time `bun:"updated_at,notnull"`
}

func (u *User) hashPassword(password string) error {
	if len(password) == 0 {
		return PasswordNotEmptyError
	}

	bytePassword := []byte(password)
	passwordHash, _ := bcrypt.GenerateFromPassword(bytePassword, bcrypt.DefaultCost)
	u.PasswordHash = string(passwordHash)

	return nil
}

func (u *User) checkPassword(password string) error {
	bytePassword := []byte(password)
	byteHashedPassword := []byte(u.PasswordHash)

	return bcrypt.CompareHashAndPassword(byteHashedPassword, bytePassword)
}

var ctx = context.Background()

func findUserByEmailOrUsername(email, username string) (*User, error) {
	database := storage.ConnectAndGetDatabase()

	var user *User

	err := database.NewSelect().Model(user).Where("email = ?", email).WhereOr("username = ?", username).Scan(ctx)

	if err != nil {
		return user, UserNotFoundError
	}

	return user, nil
}

func createUser(u *CreateUserDto) error {
	database := storage.ConnectAndGetDatabase()

	exists, err := database.NewSelect().Model((*User)(nil)).Where("email = ?", u.Email).WhereOr("username = ?", u.Username).Exists(ctx)
	if err != nil {
		return err
	}
	if exists {
		return UserAlreadyExistsError
	}

	var user User
	user.Email = u.Email
	user.Username = u.Username
	user.CreatedAt = time.Now()
	user.UpdatedAt = time.Now()
	if err := user.hashPassword(u.Password); err != nil {
		return err
	}

	if _, err := database.NewInsert().Model(&user).Exec(ctx); err != nil {
		return err
	}

	return nil
}
