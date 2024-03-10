package tokens

import (
	"time"

	"github.com/golang-jwt/jwt/v5"
	"gitlab.com/tednaaa/moner/internal/config"
)

func CreateToken(email, username string, duration time.Duration) (string, error) {
	payload, err := NewPayload(email, username, duration)
	if err != nil {
		return "", err
	}

	jwtToken := jwt.NewWithClaims(jwt.SigningMethodHS256, payload)
	signedToken, err := jwtToken.SignedString([]byte(config.Auth.Jwt_Secret))
	if err != nil {
		return "", err
	}

	return signedToken, nil
}

func VerifyToken(token string) (*Payload, error) {
	jwtToken, err := jwt.ParseWithClaims(token, &Payload{}, func(t *jwt.Token) (interface{}, error) {
		return []byte(config.Auth.Jwt_Secret), nil
	})
	if err != nil {
		return nil, err
	}

	return jwtToken.Claims.(*Payload), nil
}
