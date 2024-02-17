-- name: CreateUser :one
INSERT INTO users (
	email,
	username,
	password
) VALUES (
	$1,
	$2,
	$3
) RETURNING *;
