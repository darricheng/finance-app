package repository

import (
	"finance-app-service/models"
	"fmt"
	"os"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

var db *gorm.DB
var dbErr error

func ConnectToDb() error {
	host := os.Getenv("DB_HOST")
	user := os.Getenv("DB_USER")
	pass := os.Getenv("DB_PASS")
	name := os.Getenv("DB_NAME")
	port := os.Getenv("DB_PORT")
	sslMode := os.Getenv("DB_SSLMODE")
	dsn := fmt.Sprintf("host=%s user=%s password=%s dbname=%s port=%s sslmode=%s", host, user, pass, name, port, sslMode)
	db, dbErr = gorm.Open(postgres.Open(dsn), &gorm.Config{})

	// NOTE: The following line is not suitable for serious production apps
	// It automatically migrates schemas if I change the model
	db.AutoMigrate(&models.FinanceEntry{})

	return dbErr
}

func CreateFinanceEntry(data *models.FinanceEntry) (uint, error) {
	result := db.Create(data)
	return data.ID, result.Error
}
