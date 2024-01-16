package controllers

import (
	"net/http"

	"finance-app-service/models"

	"github.com/gin-gonic/gin"
)

func HandleDefaultRoute(c *gin.Context) {
	c.JSON(http.StatusOK, "test default endpoint success")
}

func HandleNewEntry(c *gin.Context) {
	var json models.FinanceEntry
	if err := c.ShouldBindJSON(&json); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, json)
}
