package utils

import (
	"crypto/rand"
	"fmt"
)

const alphabet = "abcdefghijklmnopqrstuvwxyz"
const minRandomString = 6

func RandomString(length int) string {
	bytes := make([]byte, length)
	_, err := rand.Read(bytes)
	if err != nil {
		return ""
	}
	for index := range bytes {
		bytes[index] = alphabet[int(bytes[index])%len(alphabet)]
	}
	return string(bytes)
}

func RandomUsername() string {
	return RandomString(minRandomString)
}

func RandomEmail() string {
	return fmt.Sprintf("%s@random.com", RandomString(minRandomString))
}
