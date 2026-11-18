// Package library owns the user-scoped, versioned Library resource and its
// append-only change feed used by desktop offline sync.
package library

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

var (
	ErrNotFound            = errors.New("library item not found")
	ErrConflict            = errors.New("library item version conflict")
	ErrDuplicate           = errors.New("library item already exists")
	ErrIdempotencyMismatch = errors.New("idempotency key reused with different request")
	ErrIdempotencyKey      = errors.New("idempotency key is required")
)

const idempotencyTTL = 24 * time.Hour

type Item struct {
	ID          uuid.UUID  `json:"id"`
	Title       string     `json:"title"`
	Description string     `json:"description"`
	Status      string     `json:"status"`
	Tags        []string   `json:"tags"`
	Version     int64      `json:"version"`
	CreatedAt   time.Time  `json:"created_at"`
	UpdatedAt   time.Time  `json:"updated_at"`
	DeletedAt   *time.Time `json:"deleted_at,omitempty"`
}

type Change struct {
	Seq       int64     `json:"seq"`
	Operation string    `json:"operation"`
	ItemID    uuid.UUID `json:"item_id"`
	Version   int64     `json:"version"`
	Item      Item      `json:"item"`
}

type Snapshot struct {
	Items  []Item `json:"items"`
	Cursor int64  `json:"cursor"`
}

type Changes struct {
	Changes []Change `json:"changes"`
	Cursor  int64    `json:"cursor"`
	HasMore bool     `json:"has_more"`
}

type CreateInput struct {
	ID          uuid.UUID
	Title       string
	Description string
	Status      string
	Tags        []string
}

type UpdateInput struct {
	Title       string
	Description string
	Status      string
	Tags        []string
}

type Mutation struct {
	Item      Item  `json:"item"`
	ChangeSeq int64 `json:"change_seq"`
}

type Result struct {
	Status   int
	Body     []byte
	Replayed bool
}

type Service struct {
	pool *pgxpool.Pool
}

func NewService(pool *pgxpool.Pool) *Service {
	return &Service{pool: pool}
}

func (s *Service) Snapshot(ctx context.Context, userID uuid.UUID) (Snapshot, error) {
	tx, err := s.pool.BeginTx(ctx, pgx.TxOptions{IsoLevel: pgx.RepeatableRead})
	if err != nil {
		return Snapshot{}, fmt.Errorf("begin snapshot: %w", err)
	}
	defer func() { _ = tx.Rollback(ctx) }()

	rows, err := tx.Query(ctx, `
		SELECT id, title, description, status, tags, version, created_at, updated_at, deleted_at
		FROM library_items
		WHERE user_id = $1 AND deleted_at IS NULL
		ORDER BY updated_at DESC`, userID)
	if err != nil {
		return Snapshot{}, fmt.Errorf("query snapshot: %w", err)
	}
	defer rows.Close()

	items := make([]Item, 0)
	for rows.Next() {
		item, scanErr := scanItem(rows)
		if scanErr != nil {
			return Snapshot{}, scanErr
		}
		items = append(items, item)
	}
	if err := rows.Err(); err != nil {
		return Snapshot{}, fmt.Errorf("iterate snapshot: %w", err)
	}

	var cursor int64
	if err := tx.QueryRow(ctx, `SELECT COALESCE(MAX(seq), 0) FROM library_changes WHERE user_id = $1`, userID).Scan(&cursor); err != nil {
		return Snapshot{}, fmt.Errorf("read snapshot cursor: %w", err)
	}
	if err := tx.Commit(ctx); err != nil {
		return Snapshot{}, fmt.Errorf("commit snapshot: %w", err)
	}

	return Snapshot{Items: items, Cursor: cursor}, nil
}

func (s *Service) Get(ctx context.Context, userID, itemID uuid.UUID) (Item, error) {
	row := s.pool.QueryRow(ctx, `
		SELECT id, title, description, status, tags, version, created_at, updated_at, deleted_at
		FROM library_items
		WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL`, itemID, userID)
	item, err := scanItem(row)
	if err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			return Item{}, ErrNotFound
		}
		return Item{}, fmt.Errorf("get library item: %w", err)
	}
	return item, nil
}

func (s *Service) Changes(ctx context.Context, userID uuid.UUID, cursor int64, limit int) (Changes, error) {
	if limit <= 0 || limit > 500 {
		limit = 200
	}
	rows, err := s.pool.Query(ctx, `
		SELECT seq, operation, item_id, version, payload
		FROM library_changes
		WHERE user_id = $1 AND seq > $2
		ORDER BY seq
		LIMIT $3`, userID, cursor, limit+1)
	if err != nil {
		return Changes{}, fmt.Errorf("query changes: %w", err)
	}
	defer rows.Close()

	result := Changes{Changes: make([]Change, 0, limit), Cursor: cursor}
	for rows.Next() {
		var change Change
		var payload []byte
		if err := rows.Scan(&change.Seq, &change.Operation, &change.ItemID, &change.Version, &payload); err != nil {
			return Changes{}, fmt.Errorf("scan change: %w", err)
		}
		if err := json.Unmarshal(payload, &change.Item); err != nil {
			return Changes{}, fmt.Errorf("decode change payload: %w", err)
		}
		if change.Seq > result.Cursor {
			result.Cursor = change.Seq
		}
		result.Changes = append(result.Changes, change)
	}
	if err := rows.Err(); err != nil {
		return Changes{}, fmt.Errorf("iterate changes: %w", err)
	}
	if len(result.Changes) > limit {
		result.HasMore = true
		result.Changes = result.Changes[:limit]
		result.Cursor = result.Changes[len(result.Changes)-1].Seq
	}

	return result, nil
}

func (s *Service) Create(ctx context.Context, userID uuid.UUID, input CreateInput, key, hash string) (Result, error) {
	return s.mutate(ctx, userID, key, hash, "create", input.ID, func(tx pgx.Tx) (Item, int64, int, error) {
		item, err := insertItem(ctx, tx, userID, input)
		if err != nil {
			return Item{}, 0, 0, err
		}
		seq, err := appendChange(ctx, tx, userID, item, "create")
		return item, seq, 201, err
	})
}

func (s *Service) Update(ctx context.Context, userID uuid.UUID, itemID uuid.UUID, version int64, input UpdateInput, key, hash string) (Result, error) {
	return s.mutate(ctx, userID, key, hash, "update", itemID, func(tx pgx.Tx) (Item, int64, int, error) {
		item, err := updateItem(ctx, tx, userID, itemID, version, input)
		if err != nil {
			return Item{}, 0, 0, err
		}
		seq, err := appendChange(ctx, tx, userID, item, "update")
		return item, seq, 200, err
	})
}

func (s *Service) Delete(ctx context.Context, userID uuid.UUID, itemID uuid.UUID, version int64, key, hash string) (Result, error) {
	return s.mutate(ctx, userID, key, hash, "delete", itemID, func(tx pgx.Tx) (Item, int64, int, error) {
		item, err := deleteItem(ctx, tx, userID, itemID, version)
		if err != nil {
			return Item{}, 0, 0, err
		}
		seq, err := appendChange(ctx, tx, userID, item, "delete")
		return item, seq, 200, err
	})
}

type mutation func(tx pgx.Tx) (Item, int64, int, error)

func (s *Service) mutate(ctx context.Context, userID uuid.UUID, key, hash, operation string, itemID uuid.UUID, apply mutation) (Result, error) {
	if key == "" {
		return Result{}, ErrIdempotencyKey
	}

	tx, err := s.pool.Begin(ctx)
	if err != nil {
		return Result{}, fmt.Errorf("begin %s mutation: %w", operation, err)
	}
	defer func() { _ = tx.Rollback(ctx) }()

	var storedHash string
	var status int
	var body []byte
	err = tx.QueryRow(ctx, `
		SELECT request_hash, status_code, response_body
		FROM idempotency_keys
		WHERE user_id = $1 AND key = $2 AND expires_at > now()`, userID, key).
		Scan(&storedHash, &status, &body)
	if err == nil {
		if storedHash != hash {
			return Result{}, ErrIdempotencyMismatch
		}
		if commitErr := tx.Commit(ctx); commitErr != nil {
			return Result{}, fmt.Errorf("commit replayed mutation: %w", commitErr)
		}
		return Result{Status: status, Body: body, Replayed: true}, nil
	}
	if !errors.Is(err, pgx.ErrNoRows) {
		return Result{}, fmt.Errorf("read idempotency key: %w", err)
	}

	item, seq, status, err := apply(tx)
	if err != nil {
		return Result{}, err
	}
	response := Mutation{Item: item, ChangeSeq: seq}
	raw, err := json.Marshal(response)
	if err != nil {
		return Result{}, fmt.Errorf("encode mutation response: %w", err)
	}
	if _, err := tx.Exec(ctx, `
		INSERT INTO idempotency_keys (user_id, key, request_hash, status_code, response_body, expires_at)
		VALUES ($1, $2, $3, $4, $5, now() + $6)`,
		userID, key, hash, status, raw, idempotencyTTL); err != nil {
		return Result{}, fmt.Errorf("store idempotency response: %w", err)
	}
	if err := tx.Commit(ctx); err != nil {
		return Result{}, fmt.Errorf("commit %s mutation: %w", operation, err)
	}

	return Result{Status: status, Body: raw}, nil
}

func insertItem(ctx context.Context, tx pgx.Tx, userID uuid.UUID, input CreateInput) (Item, error) {
	tags, err := json.Marshal(nonNilTags(input.Tags))
	if err != nil {
		return Item{}, fmt.Errorf("encode tags: %w", err)
	}
	row := tx.QueryRow(ctx, `
		INSERT INTO library_items (id, user_id, title, description, status, tags)
		VALUES ($1, $2, $3, $4, $5, $6)
		RETURNING id, title, description, status, tags, version, created_at, updated_at, deleted_at`,
		input.ID, userID, input.Title, input.Description, input.Status, tags)
	item, err := scanItem(row)
	if err != nil {
		if isUniqueViolation(err) {
			return Item{}, ErrDuplicate
		}
		return Item{}, err
	}
	return item, nil
}

func updateItem(ctx context.Context, tx pgx.Tx, userID, itemID uuid.UUID, version int64, input UpdateInput) (Item, error) {
	tags, err := json.Marshal(nonNilTags(input.Tags))
	if err != nil {
		return Item{}, fmt.Errorf("encode tags: %w", err)
	}
	row := tx.QueryRow(ctx, `
		UPDATE library_items
		SET title = $1, description = $2, status = $3, tags = $4, version = version + 1, updated_at = now()
		WHERE id = $5 AND user_id = $6 AND version = $7 AND deleted_at IS NULL
		RETURNING id, title, description, status, tags, version, created_at, updated_at, deleted_at`,
		input.Title, input.Description, input.Status, tags, itemID, userID, version)
	item, err := scanItem(row)
	if err == nil {
		return item, nil
	}
	if !errors.Is(err, pgx.ErrNoRows) {
		return Item{}, err
	}
	return Item{}, classifyMissing(ctx, tx, userID, itemID)
}

func deleteItem(ctx context.Context, tx pgx.Tx, userID, itemID uuid.UUID, version int64) (Item, error) {
	row := tx.QueryRow(ctx, `
		UPDATE library_items
		SET deleted_at = now(), updated_at = now(), version = version + 1
		WHERE id = $1 AND user_id = $2 AND version = $3 AND deleted_at IS NULL
		RETURNING id, title, description, status, tags, version, created_at, updated_at, deleted_at`,
		itemID, userID, version)
	item, err := scanItem(row)
	if err == nil {
		return item, nil
	}
	if !errors.Is(err, pgx.ErrNoRows) {
		return Item{}, err
	}
	return Item{}, classifyMissing(ctx, tx, userID, itemID)
}

func classifyMissing(ctx context.Context, tx pgx.Tx, userID, itemID uuid.UUID) error {
	var exists bool
	if err := tx.QueryRow(ctx, `SELECT EXISTS(SELECT 1 FROM library_items WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL)`, itemID, userID).Scan(&exists); err != nil {
		return fmt.Errorf("check item existence: %w", err)
	}
	if exists {
		return ErrConflict
	}
	return ErrNotFound
}

func appendChange(ctx context.Context, tx pgx.Tx, userID uuid.UUID, item Item, operation string) (int64, error) {
	payload, err := json.Marshal(item)
	if err != nil {
		return 0, fmt.Errorf("encode change payload: %w", err)
	}
	var seq int64
	if err := tx.QueryRow(ctx, `
		INSERT INTO library_changes (user_id, item_id, operation, version, payload)
		VALUES ($1, $2, $3, $4, $5)
		RETURNING seq`, userID, item.ID, operation, item.Version, payload).Scan(&seq); err != nil {
		return 0, fmt.Errorf("append change: %w", err)
	}
	return seq, nil
}

type scanner interface {
	Scan(dest ...any) error
}

func scanItem(row scanner) (Item, error) {
	var item Item
	var tags []byte
	if err := row.Scan(&item.ID, &item.Title, &item.Description, &item.Status, &tags, &item.Version, &item.CreatedAt, &item.UpdatedAt, &item.DeletedAt); err != nil {
		return Item{}, err
	}
	if err := json.Unmarshal(tags, &item.Tags); err != nil {
		return Item{}, fmt.Errorf("decode tags: %w", err)
	}
	if item.Tags == nil {
		item.Tags = []string{}
	}
	return item, nil
}

func nonNilTags(tags []string) []string {
	if tags == nil {
		return []string{}
	}
	return tags
}

func isUniqueViolation(err error) bool {
	type sqlState interface{ SQLState() string }
	var state sqlState
	return errors.As(err, &state) && state.SQLState() == "23505"
}

func RequestHash(method, path string, body []byte) string {
	sum := sha256.Sum256(append([]byte(method+"\n"+path+"\n"), body...))
	return hex.EncodeToString(sum[:])
}
