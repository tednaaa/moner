package database

import (
	"github.com/jackc/pgx/v5/pgxpool"
)

type Store struct {
	connPool *pgxpool.Pool
	*Queries
}

func NewStore(connPool *pgxpool.Pool) *Store {
	return &Store{
		connPool: connPool,
		Queries:  New(connPool),
	}
}
