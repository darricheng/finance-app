package repository

import (
	"finance-app-service/models"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

var db *gorm.DB
var dbErr error

func ConnectToDb() error {
	dsn := "host=localhost user=gorm password=gorm dbname=gorm port=9920 sslmode=disable TimeZone=Asia/Shanghai"
	db, dbErr = gorm.Open(postgres.Open(dsn), &gorm.Config{})
	return dbErr
}

func CreateFinanceEntry(data *models.FinanceEntry) (uint, error) {
	result := db.Create(data)
	return data.ID, result.Error
}
