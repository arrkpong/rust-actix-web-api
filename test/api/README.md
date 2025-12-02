# Postman Collection (test/api)

This folder contains the Postman collection for the authentication API.

- File: `postman_collection.json`
- Endpoints: health check, `/api/v1/auth/register`, `/api/v1/auth/login`
- Variables: `scheme` (http/https), `host` (default `127.0.0.1`), `port` (default `8080`)

Usage:
1) Import `postman_collection.json` into Postman.
2) Update collection variables to match your environment (`HOST`, `PORT` from `.env` if different).
3) Run requests or a collection run after starting the API.
