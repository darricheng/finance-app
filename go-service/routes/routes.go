package routes

import (
	"finance-app-service/controllers"

	"github.com/gin-gonic/gin"
)

type financeDataInput struct {
	Date struct {
		Day   int `json:"day"`
		Month int `json:"month"`
		Year  int `json:"year"`
	} `json:"date"`
	Category    string  `json:"category"`
	Amount      float64 `json:"amount"`
	Description string  `json:"description"`
}

func AddRoutes(router *gin.RouterGroup) {
	router.GET("/", controllers.DefaultController)
}
