package models

import (
	"time"

	"gorm.io/datatypes"
)

type FinanceEntry struct {
	// ISO 8601 date format, i.e. YYYY-MM-DD
	Date        datatypes.Date `json:"date" binding:"required"`
	Category    string         `json:"category" binding:"required"`
	Amount      float64        `json:"amount" binding:"required"`
	Description string         `json:"description" binding:"required"`
	// DB only fields
	ID        uint
	CreatedAt time.Time
	UpdatedAt time.Time
}
