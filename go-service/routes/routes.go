package routes

import (
	"finance-app-service/controllers"

	"github.com/gin-gonic/gin"
)

func AddRoutes(router *gin.RouterGroup) {
	router.GET("/", controllers.HandleDefaultRoute)
	router.POST("/", controllers.HandleNewEntry)
}
