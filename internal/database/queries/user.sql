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

-- name: GetUserByEmail :one
SELECT * FROM users
	WHERE email = $1;

-- name: GetUserByEmailOrUsername :one
SELECT * FROM users
	WHERE username = $1
	OR email = $2;
