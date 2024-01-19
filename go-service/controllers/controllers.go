package controllers

import (
	"net/http"
	"strings"
	"time"

	"finance-app-service/models"
	"finance-app-service/repository"

	"github.com/gin-gonic/gin"
)

func HandleDefaultRoute(c *gin.Context) {
	c.JSON(http.StatusOK, "test default endpoint success")
}

// Takes data and stores it temporarily in the storage
func HandleNewEntry(c *gin.Context) {
	var json models.FinanceEntry
	if err := c.ShouldBindJSON(&json); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	var validationErrors []string

	entryDate := json.Date
	_, dateErr := time.Parse(time.DateOnly, entryDate)
	if dateErr != nil {
		validationErrors = append(validationErrors, dateErr.Error())
	}

	if len(validationErrors) > 0 {
		c.JSON(http.StatusBadRequest, gin.H{"error": strings.Join(validationErrors, "; ")})
		return
	}

	id, dbErr := repository.CreateFinanceEntry(&json)
	if dbErr != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": dbErr.Error()})
		return
	}

	c.String(http.StatusOK, "Success, entry ID: %d", id)
	return
}

// Passes the data to the desktop app
func RetrieveEntries(c *gin.Context) {

}
