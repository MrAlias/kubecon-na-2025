// Copyright Istio Authors
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"math/rand"
	"net/http"
	"os"
	"os/signal"
	"regexp"
	"strconv"
	"sync"
	"syscall"
	"time"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

type RatingData struct {
	ID      int                    `json:"id"`
	Ratings map[string]interface{} `json:"ratings"`
}

type MongoRating struct {
	Rating int `bson:"rating"`
}

type HealthStatus struct {
	Status string `json:"status"`
}

type ErrorResponse struct {
	Error string `json:"error"`
}

var (
	userAddedRatings = make(map[int]RatingData) // used to demonstrate POST functionality
	unavailable      = false
	healthy          = true
	mu               sync.RWMutex
	mongoClient      *mongo.Client
)

func main() {
	// Get port from command line args
	port := "9080" // default port
	if len(os.Args) > 1 {
		port = os.Args[1]
	}

	// Initialize fault injection timers based on SERVICE_VERSION
	serviceVersion := os.Getenv("SERVICE_VERSION")
	initializeFaultInjection(serviceVersion)

	// Initialize MongoDB connection if needed
	if serviceVersion == "v2" {
		if err := initMongoDB(); err != nil {
			log.Printf("Failed to initialize MongoDB: %v", err)
		}
	}

	// Set up HTTP routes
	http.HandleFunc("/", handleRequest)
	http.HandleFunc("/health", handleHealth)

	// Set up graceful shutdown
	server := &http.Server{
		Addr: ":" + port,
	}

	// Channel to listen for interrupt signal
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt, syscall.SIGTERM)

	// Start server in a goroutine
	go func() {
		log.Printf("Server listening on: http://0.0.0.0:%s", port)
		if err := server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("Server failed to start: %v", err)
		}
	}()

	// Block until we receive our signal
	<-c
	log.Println("SIGTERM received")

	// Close MongoDB connection if it exists
	if mongoClient != nil {
		mongoClient.Disconnect(context.Background())
	}

	// Create a deadline to wait for
	ctx, cancel := context.WithTimeout(context.Background(), 15*time.Second)
	defer cancel()

	// Attempt graceful shutdown
	server.Shutdown(ctx)
	log.Println("Server gracefully stopped")
}

func initializeFaultInjection(serviceVersion string) {
	switch serviceVersion {
	case "v-unavailable":
		// make the service unavailable once in 60 seconds
		go func() {
			ticker := time.NewTicker(60 * time.Second)
			defer ticker.Stop()
			for range ticker.C {
				mu.Lock()
				unavailable = !unavailable
				mu.Unlock()
			}
		}()
	case "v-unhealthy":
		// make the service unavailable once in 15 minutes for 15 minutes
		go func() {
			ticker := time.NewTicker(15 * time.Minute)
			defer ticker.Stop()
			for range ticker.C {
				mu.Lock()
				healthy = !healthy
				unavailable = !unavailable
				mu.Unlock()
			}
		}()
	}
}

func initMongoDB() error {
	mongoURL := os.Getenv("MONGO_DB_URL")
	if mongoURL == "" {
		return fmt.Errorf("MONGO_DB_URL environment variable not set")
	}

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	client, err := mongo.Connect(ctx, options.Client().ApplyURI(mongoURL))
	if err != nil {
		return err
	}

	// Test the connection
	if err := client.Ping(ctx, nil); err != nil {
		return err
	}

	mongoClient = client

	// Initialize database with sample records
	if err := initializeRatingsData(ctx); err != nil {
		log.Printf("Warning: Failed to initialize ratings data: %v", err)
	}

	return nil
}

func initializeRatingsData(ctx context.Context) error {
	if mongoClient == nil {
		return fmt.Errorf("MongoDB client not initialized")
	}

	db := mongoClient.Database("test")
	collection := db.Collection("ratings")

	// Check if data already exists
	count, err := collection.CountDocuments(ctx, bson.D{})
	if err != nil {
		return fmt.Errorf("failed to count existing documents: %v", err)
	}

	// Only insert if collection is empty
	if count == 0 {
		// Insert the two rating records
		records := []interface{}{
			bson.D{{Key: "rating", Value: 5}},
			bson.D{{Key: "rating", Value: 4}},
		}

		result, err := collection.InsertMany(ctx, records)
		if err != nil {
			return fmt.Errorf("failed to insert rating records: %v", err)
		}

		log.Printf("Successfully inserted %d rating records into MongoDB", len(result.InsertedIDs))
	} else {
		log.Printf("MongoDB ratings collection already contains %d documents, skipping initialization", count)
	}

	return nil
}

func handleRequest(w http.ResponseWriter, r *http.Request) {
	log.Printf("%s %s", r.Method, r.URL.Path)

	// Match /ratings/{productId} pattern
	ratingsRegex := regexp.MustCompile(`^/ratings/(\d+)$`)
	matches := ratingsRegex.FindStringSubmatch(r.URL.Path)

	if len(matches) != 2 {
		http.NotFound(w, r)
		return
	}

	productIDStr := matches[1]
	productID, err := strconv.Atoi(productIDStr)
	if err != nil {
		writeJSONError(w, http.StatusBadRequest, "please provide numeric product ID")
		return
	}

	switch r.Method {
	case http.MethodGet:
		handleGetRatings(w, r, productID)
	case http.MethodPost:
		handlePostRatings(w, r, productID)
	default:
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
	}
}

func handleGetRatings(w http.ResponseWriter, r *http.Request, productID int) {
	log.Println("Ratings invoked")
	log.Printf("ENV %s", os.Getenv("DB_TYPE"))
	log.Printf("VER %s", os.Getenv("SERVICE_VERSION"))

	serviceVersion := os.Getenv("SERVICE_VERSION")

	if serviceVersion == "v2" {
		handleV2Ratings(w, productID)
	} else {
		handleLocalRatings(w, productID, serviceVersion)
	}
}

func handleV2Ratings(w http.ResponseWriter, productID int) {
	// Handle MongoDB
	if mongoClient == nil {
		writeJSONError(w, http.StatusInternalServerError, "could not connect to ratings database")
		return
	}

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	db := mongoClient.Database("test")
	collection := db.Collection("ratings")

	cursor, err := collection.Find(ctx, bson.D{})
	if err != nil {
		log.Printf("MongoDB find error: %v", err)
		writeJSONError(w, http.StatusInternalServerError, "could not load ratings from database")
		return
	}
	defer cursor.Close(ctx)

	var ratings []MongoRating
	if err := cursor.All(ctx, &ratings); err != nil {
		log.Printf("MongoDB cursor error: %v", err)
		writeJSONError(w, http.StatusInternalServerError, "could not load ratings from database")
		return
	}

	var firstRating, secondRating int
	if len(ratings) > 0 {
		firstRating = ratings[0].Rating
	}
	if len(ratings) > 1 {
		secondRating = ratings[1].Rating
	}

	result := RatingData{
		ID: productID,
		Ratings: map[string]interface{}{
			"Reviewer1": firstRating,
			"Reviewer2": secondRating,
		},
	}

	log.Printf("%+v", result)
	writeJSON(w, http.StatusOK, result)
}

func handleLocalRatings(w http.ResponseWriter, productID int, serviceVersion string) {
	mu.RLock()
	isUnavailable := unavailable
	mu.RUnlock()

	switch serviceVersion {
	case "v-faulty":
		// in half of the cases return error, in another half proceed as usual
		if rand.Float64() <= 0.5 {
			getLocalReviewsServiceUnavailable(w)
		} else {
			getLocalReviewsSuccessful(w, productID)
		}
	case "v-delayed":
		// in half of the cases delay for 7 seconds, in another half proceed as usual
		if rand.Float64() <= 0.5 {
			time.Sleep(7 * time.Second)
			getLocalReviewsSuccessful(w, productID)
		} else {
			getLocalReviewsSuccessful(w, productID)
		}
	case "v-unavailable", "v-unhealthy":
		if isUnavailable {
			getLocalReviewsServiceUnavailable(w)
		} else {
			getLocalReviewsSuccessful(w, productID)
		}
	default:
		getLocalReviewsSuccessful(w, productID)
	}
}

func handlePostRatings(w http.ResponseWriter, r *http.Request, productID int) {
	var ratings map[string]interface{}
	if err := json.NewDecoder(r.Body).Decode(&ratings); err != nil {
		writeJSONError(w, http.StatusBadRequest, "please provide valid ratings JSON")
		return
	}

	serviceVersion := os.Getenv("SERVICE_VERSION")
	if serviceVersion == "v2" {
		// the version that is backed by a database
		writeJSONError(w, http.StatusNotImplemented, "Post not implemented for database backed ratings")
	} else {
		// the version that holds ratings in-memory
		result := putLocalReviews(productID, ratings)
		writeJSON(w, http.StatusOK, result)
	}
}

func handleHealth(w http.ResponseWriter, r *http.Request) {
	mu.RLock()
	isHealthy := healthy
	mu.RUnlock()

	if isHealthy {
		writeJSON(w, http.StatusOK, HealthStatus{Status: "Ratings is healthy"})
	} else {
		writeJSON(w, http.StatusInternalServerError, HealthStatus{Status: "Ratings is not healthy"})
	}
}

func putLocalReviews(productID int, ratings map[string]interface{}) RatingData {
	mu.Lock()
	defer mu.Unlock()

	userAddedRatings[productID] = RatingData{
		ID:      productID,
		Ratings: ratings,
	}
	return getLocalReviews(productID)
}

func getLocalReviewsSuccessful(w http.ResponseWriter, productID int) {
	result := getLocalReviews(productID)
	writeJSON(w, http.StatusOK, result)
}

func getLocalReviewsServiceUnavailable(w http.ResponseWriter) {
	writeJSONError(w, http.StatusServiceUnavailable, "Service unavailable")
}

func getLocalReviews(productID int) RatingData {
	mu.RLock()
	defer mu.RUnlock()

	if rating, exists := userAddedRatings[productID]; exists {
		return rating
	}

	return RatingData{
		ID: productID,
		Ratings: map[string]interface{}{
			"Reviewer1": 5,
			"Reviewer2": 4,
		},
	}
}

func writeJSON(w http.ResponseWriter, statusCode int, data interface{}) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(statusCode)
	json.NewEncoder(w).Encode(data)
}

func writeJSONError(w http.ResponseWriter, statusCode int, message string) {
	writeJSON(w, statusCode, ErrorResponse{Error: message})
}
