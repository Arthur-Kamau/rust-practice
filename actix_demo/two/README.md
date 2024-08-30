# ACTIX state management
1. POST `curl -X POST -H "Content-Type: application/json" -d '{"id": 1, "name": "Item 1"}' http://localhost:8080/items`
2. GET `curl http://localhost:8080/items`
3. DELETE `curl -X DELETE http://localhost:8080/items/1`
4. PUT `curl -X PUT -H "Content-Type: application/json" -d '{"id": 1, "name": "Updated Item 1"}' http://localhost:8080/items`


