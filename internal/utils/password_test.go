package utils_test

import (
	"testing"

	"github.com/stretchr/testify/require"
	"gitlab.com/tednaaa/moner/internal/utils"
	"golang.org/x/crypto/bcrypt"
)

func TestPassword(t *testing.T) {
	password := utils.RandomString(10)

	hashedPassword, err := utils.HashPassword(password)
	require.NoError(t, err)
	require.NotEmpty(t, hashedPassword)

	err = utils.CheckPassword(password, hashedPassword)
	require.NoError(t, err)
}
func TestWrongPassword(t *testing.T) {
	password := utils.RandomString(10)

	hashedPassword, err := utils.HashPassword(password)
	require.NoError(t, err)
	require.NotEmpty(t, hashedPassword)

	err = utils.CheckPassword("wrong_password", hashedPassword)
	require.ErrorIs(t, err, bcrypt.ErrMismatchedHashAndPassword)
}
