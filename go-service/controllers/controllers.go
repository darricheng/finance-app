package controllers

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

type financeEntry struct {
	Date struct {
		Day   int `json:"day" binding:"required"`
		Month int `json:"month" binding:"required"`
		Year  int `json:"year" binding:"required"`
	} `json:"date"`
	Category    string  `json:"category" binding:"required"`
	Amount      float64 `json:"amount" binding:"required"`
	Description string  `json:"description" binding:"required"`
}

func HandleDefaultRoute(c *gin.Context) {
	c.JSON(http.StatusOK, "test default endpoint success")
}

func HandleNewEntry(c *gin.Context) {
	var json financeEntry
	if err := c.ShouldBindJSON(&json); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, json)
}
