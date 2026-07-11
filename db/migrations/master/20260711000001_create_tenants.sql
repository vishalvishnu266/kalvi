CREATE TABLE tenants (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    contact_email TEXT NOT NULL,
    contact_phone TEXT NOT NULL,
    address TEXT NOT NULL,
    database_name TEXT NOT NULL UNIQUE,
    primary_color TEXT NOT NULL DEFAULT '#4f46e5',
    dark_mode INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
