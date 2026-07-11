-- Seed Script: Create Default Admin User
-- Run this against your tenant database after onboarding
-- Example: sqlite3 tenant_demo-school.db < SEED_ADMIN.sql

-- Password: admin123 (bcrypt hash with cost 12)
INSERT OR IGNORE INTO users (username, email, password_hash, role, is_active) 
VALUES (
    'admin', 
    'admin@school.edu', 
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5yvU1MQl8xvLu', 
    'admin',
    1
);

-- Create a teacher user for testing
INSERT OR IGNORE INTO users (username, email, password_hash, role, is_active) 
VALUES (
    'teacher1', 
    'teacher@school.edu', 
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5yvU1MQl8xvLu', 
    'teacher',
    1
);

-- Create a staff user for testing
INSERT OR IGNORE INTO users (username, email, password_hash, role, is_active) 
VALUES (
    'staff1', 
    'staff@school.edu', 
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5yvU1MQl8xvLu', 
    'staff',
    1
);

-- Display created users
SELECT 
    id,
    username,
    email,
    role,
    is_active,
    created_at
FROM users;

-- ⚠️ SECURITY WARNING ⚠️
-- All users created with password: admin123
-- CHANGE PASSWORDS IMMEDIATELY IN PRODUCTION!
