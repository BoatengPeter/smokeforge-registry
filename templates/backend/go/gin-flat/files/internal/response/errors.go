// Package response owns HTTP success responses and RFC 9457 error responses.
package response

import (
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"strings"
	"unicode"

	"github.com/gin-gonic/gin"
	"github.com/go-playground/validator/v10"
	"go.uber.org/zap"
)

type contextKey string

const requestIDKey contextKey = "request_id"

// SetRequestID stores the server-generated request ID on the Gin context.
func SetRequestID(c *gin.Context, requestID string) {
	c.Set(string(requestIDKey), requestID)
}

// RequestID returns the server-generated request ID.
func RequestID(c *gin.Context) string {
	value, ok := c.Get(string(requestIDKey))
	if !ok {
		return ""
	}

	requestID, _ := value.(string)

	return requestID
}

// FieldError is one field-level validation error.
type FieldError struct {
	Field   string `json:"field"`
	Message string `json:"message"`
}

// ErrorResponse is an RFC 9457 error response with stable extensions.
type ErrorResponse struct {
	Type      string       `json:"type"`
	Title     string       `json:"title"`
	Status    int32        `json:"status"`
	Detail    string       `json:"detail,omitempty"`
	Instance  string       `json:"instance,omitempty"`
	Code      string       `json:"code,omitempty"`
	RequestID string       `json:"request_id,omitempty"`
	Errors    []FieldError `json:"errors,omitempty"`
}

// JSON writes a raw JSON response that matches the OpenAPI contract.
func JSON(c *gin.Context, status int, payload any) {
	c.JSON(status, payload)
}

// OK writes a 200 response.
func OK(c *gin.Context, payload any) {
	JSON(c, http.StatusOK, payload)
}

// Created writes a 201 response.
func Created(c *gin.Context, payload any) {
	JSON(c, http.StatusCreated, payload)
}

// Accepted writes a 202 response.
func Accepted(c *gin.Context, payload any) {
	JSON(c, http.StatusAccepted, payload)
}

// NoContent writes a 204 response.
func NoContent(c *gin.Context) {
	c.Status(http.StatusNoContent)
}

// NewError creates an RFC 9457 error response.
func NewError(status int, code, title, detail string) ErrorResponse {
	return ErrorResponse{
		Type:   "about:blank",
		Title:  title,
		Status: errorStatus(status),
		Detail: detail,
		Code:   code,
	}
}

// WriteError writes an RFC 9457 error response and aborts the Gin chain.
func WriteError(c *gin.Context, payload ErrorResponse) {
	if payload.Type == "" {
		payload.Type = "about:blank"
	}

	if payload.Status < 100 || payload.Status > 599 {
		payload.Status = http.StatusInternalServerError
	}

	if payload.RequestID == "" {
		payload.RequestID = RequestID(c)
	}

	if payload.Instance == "" {
		payload.Instance = c.Request.URL.Path
	}

	body, err := json.Marshal(payload)
	if err != nil {
		body = []byte(`{"type":"about:blank","title":"Internal Server Error","status":500}`)
	}

	c.Header("Content-Type", "application/problem+json; charset=utf-8")
	c.Status(int(payload.Status))
	_, _ = c.Writer.Write(body)
	c.Abort()
}

// WriteValidationError maps validator errors to a 422 error response.
func WriteValidationError(c *gin.Context, err error) {
	var validationErrors validator.ValidationErrors
	if !errors.As(err, &validationErrors) {
		WriteError(c, NewError(http.StatusBadRequest, "invalid_request", "Invalid request", "The request could not be parsed."))

		return
	}

	fields := make([]FieldError, 0, len(validationErrors))
	for _, fieldError := range validationErrors {
		fields = append(fields, FieldError{
			Field:   fieldName(fieldError.StructField()),
			Message: validationMessage(fieldError),
		})
	}

	payload := NewError(http.StatusUnprocessableEntity, "validation_failed", "Validation failed", "One or more fields are invalid.")
	payload.Errors = fields

	WriteError(c, payload)
}

// WriteInternalError logs an unexpected error without exposing details.
func WriteInternalError(c *gin.Context, logger *zap.Logger, err error) {
	fields := []zap.Field{zap.Error(err)}
	if requestID := RequestID(c); requestID != "" {
		fields = append(fields, zap.String("request_id", requestID))
	}

	logger.Error("request failed", fields...)

	WriteError(c, NewError(http.StatusInternalServerError, "internal_error", "Internal server error", "The request could not be completed."))
}

// WriteUnauthorized writes a 401 error response.
func WriteUnauthorized(c *gin.Context, detail string) {
	WriteError(c, NewError(http.StatusUnauthorized, "unauthorized", "Unauthorized", detail))
}

// WriteForbidden writes a 403 error response.
func WriteForbidden(c *gin.Context, detail string) {
	WriteError(c, NewError(http.StatusForbidden, "forbidden", "Forbidden", detail))
}

// WriteNotFound writes a 404 error response.
func WriteNotFound(c *gin.Context, detail string) {
	WriteError(c, NewError(http.StatusNotFound, "not_found", "Not found", detail))
}

// WriteConflict writes a 409 error response.
func WriteConflict(c *gin.Context, code, detail string) {
	WriteError(c, NewError(http.StatusConflict, code, "Conflict", detail))
}

// WriteTooManyRequests writes a 429 error response.
func WriteTooManyRequests(c *gin.Context) {
	WriteError(c, NewError(http.StatusTooManyRequests, "rate_limited", "Too many requests", "The request rate limit was exceeded."))
}

func errorStatus(status int) int32 {
	if status < 100 || status > 599 {
		return http.StatusInternalServerError
	}

	return int32(status)
}

func validationMessage(err validator.FieldError) string {
	switch err.Tag() {
	case "required":
		return "This field is required."
	case "email":
		return "Must be a valid email address."
	case "min":
		return fmt.Sprintf("Must be at least %s characters.", err.Param())
	case "max":
		return fmt.Sprintf("Must be at most %s characters.", err.Param())
	case "uuid":
		return "Must be a valid UUID."
	default:
		return "This field is invalid."
	}
}

func fieldName(name string) string {
	var builder strings.Builder

	for index, char := range name {
		if unicode.IsUpper(char) {
			if index > 0 {
				builder.WriteByte('_')
			}

			builder.WriteRune(unicode.ToLower(char))

			continue
		}

		builder.WriteRune(char)
	}

	return builder.String()
}
