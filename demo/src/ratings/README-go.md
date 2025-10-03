# Ratings Service - Go Implementation

This is a Go rewrite of the original Node.js ratings service using the official MongoDB Go driver.

## Features

- **HTTP API**: 
  - `GET /ratings/{productId}` - Get ratings for a product
  - `POST /ratings/{productId}` - Add ratings for a product (in-memory version only)
  - `GET /health` - Health check endpoint

- **Service Versions**:
  - `v2`: Uses MongoDB for persistent storage
  - Default: Uses in-memory storage with fault injection capabilities

- **Fault Injection Modes**:
  - `v-faulty`: Returns errors 50% of the time
  - `v-delayed`: Adds 7-second delay 50% of the time
  - `v-unavailable`: Toggles availability every 60 seconds
  - `v-unhealthy`: Toggles health every 15 minutes

## Environment Variables

- `SERVICE_VERSION`: Controls service behavior (`v2`, `v-faulty`, `v-delayed`, `v-unavailable`, `v-unhealthy`)
- `MONGO_DB_URL`: MongoDB connection string (required for v2)
- `DB_TYPE`: Database type (only `mongodb` supported in this Go version)

## Building and Running

### Local Development
```bash
go mod tidy
go run main.go [port]
```

### Docker Build
```bash
docker build -f Dockerfile.go -t ratings-go:local .
```

### Running with Docker
```bash
# In-memory version
docker run -p 9080:9080 ratings-go:local

# MongoDB version
docker run -p 9080:9080 \
  -e SERVICE_VERSION=v2 \
  -e MONGO_DB_URL=mongodb://mongodb:27017/test \
  ratings-go:local
```

## API Examples

### Get Ratings
```bash
curl http://localhost:9080/ratings/123
```

Response:
```json
{
  "id": 123,
  "ratings": {
    "Reviewer1": 5,
    "Reviewer2": 4
  }
}
```

### Add Ratings (in-memory version only)
```bash
curl -X POST http://localhost:9080/ratings/123 \
  -H "Content-Type: application/json" \
  -d '{"Reviewer1": 3, "Reviewer2": 5}'
```

### Health Check
```bash
curl http://localhost:9080/health
```

## Differences from Node.js Version

1. **No MySQL Support**: As requested, MySQL functionality was not implemented
2. **Better Concurrency**: Uses Go's goroutines for better concurrent request handling
3. **Type Safety**: Strongly typed implementation with proper error handling
4. **Graceful Shutdown**: Implements proper signal handling for graceful shutdown
5. **MongoDB Driver**: Uses the official MongoDB Go driver instead of the Node.js client

## Dependencies

- `go.mongodb.org/mongo-driver`: Official MongoDB driver for Go
- Standard Go library for HTTP server and JSON handling