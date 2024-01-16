package controllers

import (
	"net/http"

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

	id, err := repository.CreateFinanceEntry(&json)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
	}

	c.String(http.StatusOK, "Success, entry ID: %d", id)
}

// Passes the data to the desktop app
func RetrieveEntries(c *gin.Context) {

}
