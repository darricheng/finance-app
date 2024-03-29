package routes

import (
	"finance-app-service/controllers"

	"github.com/gin-gonic/gin"
)

func AddRoutes(router *gin.RouterGroup) {
	router.GET("/test", controllers.TestEndpoint)
	router.GET("", controllers.RetrieveEntries)
	router.POST("", controllers.HandleNewEntry)
}
