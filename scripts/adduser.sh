
curl -X POST http://127.0.0.1:3000/api/tenant/auth/register \
  -H 'content-type: application/json' \
  -H 'x-tenant-id: acme' \
  -d '{
    "username": "admin",
    "email":    "admin@acme.example",
    "password": "admin123",
    "roles":    ["admin"]
  }'



curl -X POST http://127.0.0.1:3000/api/tenant/auth/register \
  -H 'content-type: application/json' \
  -H 'x-tenant-id: globex' \
  -d '{"username":"admin","email":"admin@globex.example","password":"admin123","roles":["admin"]}'