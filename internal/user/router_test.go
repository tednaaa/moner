package user_test

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/tednaaa/moner/internal/router"
	"github.com/tednaaa/moner/internal/storage"
	"github.com/tednaaa/moner/internal/user"
)

func TestGetUserRoute(t *testing.T) {
	storage.ClearTable("users", (*user.User)(nil))
	router := router.SetupRouter()

	user := user.CreateUserDto{
		Email:    "user@example.com",
		Username: "username",
		Password: "password_1234",
	}
	userJson, _ := json.Marshal(user)
	request, _ := http.NewRequest("POST", "/api/user", bytes.NewBuffer(userJson))

	w := httptest.NewRecorder()
	router.ServeHTTP(w, request)

	assert.Equal(t, http.StatusCreated, w.Code)
}
