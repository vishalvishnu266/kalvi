#!/usr/bin/env bash
# Register a default admin user in each demo tenant.
#
# Tenancy is path-based: the tenant id appears in the URL as
# `/api/tenant/{tenant}/…` — no `x-tenant-id` header needed.

curl -X POST http://127.0.0.1:3000/api/tenant/acme/auth/register \
  -H 'content-type: application/json' \
  -d '{
    "username": "admin",
    "email":    "admin@acme.example",
    "password": "admin123",
    "roles":    ["admin"]
  }'

curl -X POST http://127.0.0.1:3000/api/tenant/globex/auth/register \
  -H 'content-type: application/json' \
  -d '{"username":"admin","email":"admin@globex.example","password":"admin123","roles":["admin"]}'
