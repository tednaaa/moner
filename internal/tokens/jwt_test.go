package tokens_test

import (
	"testing"
	"time"

	"github.com/golang-jwt/jwt/v5"
	"github.com/stretchr/testify/require"
	"gitlab.com/tednaaa/moner/internal/tokens"
	"gitlab.com/tednaaa/moner/internal/utils"
)

func TestJWT(t *testing.T) {
	jwtMaker, err := tokens.NewJWTMaker(utils.RandomString(tokens.MinSecretKeySize))
	require.NoError(t, err)

	email := utils.RandomEmail()
	username := utils.RandomUsername()
	duration := time.Minute

	issuedAt := time.Now()
	notBefore := time.Now()
	expiredAt := issuedAt.Add(duration)

	token, err := jwtMaker.CreateToken(email, username, duration)
	require.NoError(t, err)
	require.NotEmpty(t, token)

	payload, err := jwtMaker.VerifyToken(token)

	require.NoError(t, err)
	require.NotEmpty(t, payload)

	require.NotZero(t, payload.ID)
	require.Equal(t, email, payload.Email)
	require.Equal(t, username, payload.Username)
	require.WithinDuration(t, issuedAt, payload.IssuedAt.Time, time.Second)
	require.WithinDuration(t, notBefore, payload.NotBefore.Time, time.Second)
	require.WithinDuration(t, expiredAt, payload.ExpiresAt.Time, time.Second)
}

func TestExpiredJWTToken(t *testing.T) {
	jwtMaker, err := tokens.NewJWTMaker(utils.RandomString(tokens.MinSecretKeySize))
	require.NoError(t, err)

	token, err := jwtMaker.CreateToken(utils.RandomEmail(), utils.RandomUsername(), -time.Minute)
	require.NoError(t, err)
	require.NotEmpty(t, token)

	payload, err := jwtMaker.VerifyToken(token)
	require.Error(t, err)
	require.ErrorIs(t, err, jwt.ErrTokenExpired)
	require.Nil(t, payload)
}

func TestInvalidJWTTokenAlgNone(t *testing.T) {
	jwtMaker, err := tokens.NewJWTMaker(utils.RandomString(tokens.MinSecretKeySize))
	require.NoError(t, err)

	payload, err := tokens.NewPayload(utils.RandomEmail(), utils.RandomUsername(), time.Minute)
	require.NoError(t, err)

	jwtToken := jwt.NewWithClaims(jwt.SigningMethodNone, payload)
	token, err := jwtToken.SignedString(jwt.UnsafeAllowNoneSignatureType)
	require.NoError(t, err)

	payload, err = jwtMaker.VerifyToken(token)
	require.Error(t, err)
	require.ErrorIs(t, err, jwt.ErrTokenUnverifiable)
	require.Nil(t, payload)
}
