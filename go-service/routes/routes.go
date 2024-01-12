package routes

import (
	"finance-app-service/controllers"

	"github.com/gin-gonic/gin"
)

type Login struct {
	User     string `form:"user" json:"user" xml:"user"  binding:"required"`
	Password string `form:"password" json:"password" xml:"password" binding:"required"`
}

type SomeStruct struct {
	SomeField string `json:"somefield" binding:"required"`
}

func AddRoutes(router *gin.RouterGroup) {
	router.GET("/", controllers.HandleDefaultRoute)
	router.POST("/new-entry", controllers.HandleNewEntry)
}
