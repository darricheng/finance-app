package controllers

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func DefaultController(c *gin.Context) {
	c.JSON(http.StatusOK, "test default endpoint success")
}
