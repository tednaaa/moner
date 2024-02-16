package user_test

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/gin-gonic/gin"
	"github.com/stretchr/testify/assert"
	"gitlab.com/tednaaa/moner/internal/router"
	"gitlab.com/tednaaa/moner/internal/storage"
	"gitlab.com/tednaaa/moner/internal/user"
	"gitlab.com/tednaaa/moner/internal/utils"
)

func TestGetUserRoute(t *testing.T) {
	gin.SetMode(gin.TestMode)
	err := utils.LoadConfig("../..")
	if err != nil {
		fmt.Println(err)
	}

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
