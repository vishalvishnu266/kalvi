# Student Module

This module will contain all student-related features for the School ERP system.

## Status: 🚧 Ready for Development

Test endpoints have been removed. This module is now a clean slate ready for real student management features.

## Planned Features

### 1. Student List/Dashboard
- View all students in tenant
- Search and filter functionality
- Pagination
- Export to CSV

### 2. Student Profile
- View complete student details
- Display enrollment information
- Show academic history
- Contact information

### 3. Add/Edit Student
- Form to add new students
- Edit existing student details
- Validation (email, phone, dates)
- Photo upload (future)

### 4. Student Enrollment
- Enroll in classes
- Academic year management
- Grade level tracking

## Database Schema

Current `students` table in tenant database:

```sql
CREATE TABLE students (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    date_of_birth TEXT,
    enrollment_date TEXT NOT NULL DEFAULT (date('now')),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

### Future Fields to Add
- `student_id` - Unique student identifier (e.g., "STU2024001")
- `gender` - Student gender
- `address` - Home address
- `guardian_name` - Parent/guardian name
- `guardian_phone` - Emergency contact
- `guardian_email` - Parent email
- `grade_level` - Current grade
- `section` - Class section
- `photo_url` - Profile photo
- `is_active` - Active enrollment status
- `admission_date` - When admitted
- `notes` - Additional information

## Implementation Guidelines

### Use Hotwire Turbo
Refer to `HOTWIRE_REFERENCE.md` for implementation patterns:
- Turbo Frames for inline editing
- Turbo Streams for live updates
- Progressive enhancement

### Follow Feature-Based Organization
Each feature in its own file:
```
student/src/
├── lib.rs              # Routes export
├── shared.rs           # Shared models & DB queries
├── list_students.rs    # List/search students
├── view_student.rs     # Student profile
├── add_student.rs      # Add new student
├── edit_student.rs     # Edit student (with Turbo)
└── enrollment.rs       # Enrollment features
```

### Database Operations Pattern
```rust
// In shared.rs
pub mod db {
    pub async fn get_student_by_id(pool: &SqlitePool, id: i64) 
        -> Result<Option<Student>, sqlx::Error> {
        // Implementation
    }
    
    pub async fn list_students(pool: &SqlitePool) 
        -> Result<Vec<Student>, sqlx::Error> {
        // Implementation
    }
}
```

### Route Pattern
```rust
// In feature file
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/t/{slug}/students", get(list_students_handler))
        .route("/t/{slug}/student/{id}", get(view_student_handler))
        .route("/t/{slug}/student/add", get(show_add_form).post(add_student_handler))
}
```

## Next Steps

1. **Start with Student List**
   - Create `list_students.rs`
   - Show all students in table
   - Add search/filter

2. **Add Student Profile**
   - Create `view_student.rs`
   - Show detailed student info
   - Use Bootstrap cards

3. **Implement Add Student**
   - Create `add_student.rs`
   - Form with validation
   - Success message with Turbo

4. **Add Edit Functionality**
   - Create `edit_student.rs`
   - Inline editing with Turbo Frames
   - Update with Turbo Streams

---

**Reference:** See `HOTWIRE_REFERENCE.md` for Turbo implementation patterns
