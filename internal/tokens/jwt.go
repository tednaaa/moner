package tokens

import (
	"fmt"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

const MinSecretKeySize = 32

type JWTMaker struct {
	secretKey string
}

func NewJWTMaker(secretKey string) (*JWTMaker, error) {
	if len(secretKey) < MinSecretKeySize {
		return nil, fmt.Errorf("invalid key size: must be at least %d characters", MinSecretKeySize)
	}

	return &JWTMaker{secretKey}, nil
}

func (j *JWTMaker) CreateToken(email, username string, duration time.Duration) (string, error) {
	payload, err := NewPayload(email, username, duration)
	if err != nil {
		return "", err
	}

	jwtToken := jwt.NewWithClaims(jwt.SigningMethodHS256, payload)
	signedToken, err := jwtToken.SignedString([]byte(j.secretKey))
	if err != nil {
		return "", fmt.Errorf("failed to create token: %w", err)
	}

	return signedToken, nil
}

func (j *JWTMaker) VerifyToken(token string) (*Payload, error) {
	jwtToken, err := jwt.ParseWithClaims(token, &Payload{}, func(_ *jwt.Token) (interface{}, error) {
		return []byte(j.secretKey), nil
	})
	if err != nil {
		return nil, fmt.Errorf("invalid token: %w", err)
	}

	return jwtToken.Claims.(*Payload), nil
}
