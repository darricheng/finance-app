package main

import (
	"finance-app-service/routes"

	"github.com/gin-gonic/gin"
)

func main() {
	router := gin.Default()
	v1 := router.Group("/v1")
	routes.AddRoutes(v1)
	router.Run()
}
