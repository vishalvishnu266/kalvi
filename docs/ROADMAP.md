# School ERP - Next Steps Roadmap

## Current Status ✅

### What's Complete
- ✅ **Tenant Onboarding** - Multi-tenant system with isolated databases
- ✅ **Custom Session System** - Production-ready, tenant-scoped sessions
- ✅ **User Authentication** - Login/logout with bcrypt, role-based access
- ✅ **Modern UI** - Beautiful, responsive design with Bootstrap 5 & icons
- ✅ **Path-based Routing** - `/t/{slug}/...` for all tenant features
- ✅ **Complete Isolation** - Users, sessions, data all isolated per tenant

### Database Schema
```
Master DB (master.db):
  - tenants table

Tenant DB (tenant_{slug}.db):
  - users table (username, email, password_hash, role)
  - sessions table (id, user_id, created_at, expires_at, ip, user_agent)
  - students table (ready for expansion)
```

### Available URLs
- `GET /` - Landing page
- `GET /onboard` - Tenant onboarding
- `GET /t/{slug}/login` - Tenant login
- `GET /t/{slug}/dashboard` - User dashboard
- `GET /t/{slug}/logout` - Logout

---

## Phase 1: Core Student Management 🎓

### Priority: HIGH
Build the core student management features - the heart of the ERP system.

### 1.1 Student List Page
**Route:** `GET /t/{slug}/students`

**Features:**
- Display all students in a table
- Search by name, email, or ID
- Filter by grade level, status
- Pagination (20 students per page)
- Export to CSV
- "Add New Student" button

**UI Elements:**
```
┌─────────────────────────────────────────────┐
│ Students                    [+ Add Student] │
├─────────────────────────────────────────────┤
│ Search: [___________]  Grade: [All ▼]       │
├─────────────────────────────────────────────┤
│ ID  │ Name          │ Email        │ Grade │
├─────┼───────────────┼──────────────┼───────┤
│ 001 │ John Doe      │ john@...     │ 10    │
│ 002 │ Jane Smith    │ jane@...     │ 11    │
└─────────────────────────────────────────────┘
```

**Database Updates:**
```sql
-- Expand students table
ALTER TABLE students ADD COLUMN student_id TEXT UNIQUE;
ALTER TABLE students ADD COLUMN grade_level INTEGER;
ALTER TABLE students ADD COLUMN status TEXT DEFAULT 'active';
ALTER TABLE students ADD COLUMN guardian_name TEXT;
ALTER TABLE students ADD COLUMN guardian_phone TEXT;
ALTER TABLE students ADD COLUMN guardian_email TEXT;
ALTER TABLE students ADD COLUMN address TEXT;
ALTER TABLE students ADD COLUMN admission_date TEXT;
ALTER TABLE students ADD COLUMN photo_url TEXT;
```

**Implementation:**
- Create `server/crates/student/src/list_students.rs`
- Use Hotwire Turbo for search/filter (no page reload)
- Bootstrap table with hover effects
- Icons for actions (view, edit, delete)

### 1.2 Add Student Form
**Route:** `GET /t/{slug}/students/new`

**Features:**
- Multi-step form (Personal Info → Guardian Info → Academic Info)
- Real-time validation
- Photo upload (optional)
- Generate student ID automatically
- Success message with Turbo Stream

**Form Fields:**
```
Personal Information:
  - Name (required)
  - Date of Birth (required)
  - Gender (required)
  - Email
  - Phone
  - Address

Guardian Information:
  - Guardian Name (required)
  - Guardian Phone (required)
  - Guardian Email
  - Relationship

Academic Information:
  - Grade Level (required)
  - Admission Date (default: today)
  - Student ID (auto-generated)
```

**Implementation:**
- Use Turbo Frames for multi-step form
- Client-side validation + server-side validation
- Photo upload to `/uploads/{tenant_slug}/students/`
- Generate student ID: `STU{year}{sequence}` (e.g., STU2024001)

### 1.3 View Student Profile
**Route:** `GET /t/{slug}/students/{id}`

**Features:**
- Full student details in card layout
- Photo display
- Quick actions (Edit, Delete, Print)
- Tabs for: Info, Attendance, Grades, Documents
- Activity timeline

**UI Layout:**
```
┌─────────────────────────────────────────────┐
│ [Photo]  John Doe                          │
│          Student ID: STU2024001            │
│          Grade: 10    Status: Active       │
├─────────────────────────────────────────────┤
│ [Info] [Attendance] [Grades] [Documents]   │
├─────────────────────────────────────────────┤
│ Personal Information                        │
│ Email: john@example.com                    │
│ Phone: +1234567890                         │
│ ...                                        │
└─────────────────────────────────────────────┘
```

### 1.4 Edit Student
**Route:** `GET /t/{slug}/students/{id}/edit`

**Features:**
- Inline editing with Turbo Frames
- Same form as "Add Student" but pre-populated
- Update with Turbo Stream
- Change history tracking

**Hotwire Pattern:**
```rust
// View mode shows student info with "Edit" button
turbo-frame#student-info {
    // Student details
    <a href="/t/{slug}/students/{id}/edit">Edit</a>
}

// Edit mode replaces with form
turbo-frame#student-info {
    <form action="/t/{slug}/students/{id}" method="post">
        // Editable fields
    </form>
}

// On submit, Turbo Stream updates
turbo-stream action="replace" target="student-info" {
    // Updated student details
}
```

### 1.5 Delete Student
**Route:** `POST /t/{slug}/students/{id}/delete`

**Features:**
- Soft delete (set status = 'deleted')
- Confirmation modal
- Archive instead of permanent delete
- Restore functionality for admins

---

## Phase 2: User Management 👥

### Priority: HIGH
Allow admins to create and manage users (teachers, staff).

### 2.1 User List (Admin Only)
**Route:** `GET /t/{slug}/admin/users`

**Features:**
- List all users
- Filter by role (Admin, Teacher, Staff)
- Search by username/email
- Activate/deactivate users
- View active sessions per user

### 2.2 Create User
**Route:** `GET /t/{slug}/admin/users/new`

**Features:**
- Username, email, password
- Role selection (Admin, Teacher, Staff)
- Auto-generate password option
- Email invitation (future)

### 2.3 Edit User
**Route:** `GET /t/{slug}/admin/users/{id}/edit`

**Features:**
- Update username, email, role
- Reset password
- Activate/deactivate
- View user's login history

---

## Phase 3: Session Management (Admin Panel) 🔐

### Priority: MEDIUM
Build the admin control panel for session management.

### 3.1 Active Sessions Dashboard
**Route:** `GET /t/{slug}/admin/sessions`

**Features:**
- List all active sessions
- Show: User, IP, Device, Last Activity
- Force logout any session
- Revoke all sessions for a user
- Session statistics (total, by role, by device)

**UI:**
```
┌─────────────────────────────────────────────┐
│ Active Sessions                    Total: 5 │
├─────────────────────────────────────────────┤
│ User      │ IP          │ Device   │ Logout │
├───────────┼─────────────┼──────────┼────────┤
│ admin     │ 192.168.1.1 │ Chrome   │ [X]    │
│ teacher1  │ 192.168.1.2 │ Safari   │ [X]    │
└─────────────────────────────────────────────┘
```

**Implementation:**
```rust
// Use SessionManager
let sessions = session_manager.get_all_active_sessions().await?;

// Render table with Turbo Stream for live updates
```

### 3.2 Session Limits
**Feature:** Enforce max sessions per user

**Implementation:**
```rust
// In SessionManager
const MAX_SESSIONS_PER_USER: usize = 3;

// On login
session_manager.enforce_session_limit(user_id, MAX_SESSIONS_PER_USER).await?;
```

### 3.3 Login History
**Route:** `GET /t/{slug}/admin/users/{id}/sessions`

**Features:**
- Show all sessions (active and expired) for a user
- Login timestamps, IPs, devices
- Suspicious activity alerts
- Export to CSV

**Database:**
```sql
-- Optional: Keep session history
CREATE TABLE session_history (
    id INTEGER PRIMARY KEY,
    session_id TEXT,
    user_id INTEGER,
    action TEXT,  -- 'created', 'renewed', 'expired', 'revoked'
    created_at TEXT,
    ip_address TEXT,
    user_agent TEXT
);
```

---

## Phase 4: Enhanced Dashboard 📊

### Priority: MEDIUM
Improve the dashboard with real data and charts.

### 4.1 Statistics Cards
- Total Students (with trend)
- Active Users
- Today's Attendance Rate
- Upcoming Events

### 4.2 Quick Actions
- Add Student (modal)
- Mark Attendance
- Generate Report
- View Announcements

### 4.3 Recent Activity
- Latest student enrollments
- Recent login activity
- System notifications

### 4.4 Charts (Optional)
- Student enrollment trend (Chart.js)
- Attendance by grade
- User activity heatmap

---

## Phase 5: Attendance System 📅

### Priority: MEDIUM
Track daily student attendance.

### 5.1 Database Schema
```sql
CREATE TABLE attendance (
    id INTEGER PRIMARY KEY,
    student_id INTEGER NOT NULL,
    date TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('present', 'absent', 'late', 'excused')),
    marked_by INTEGER NOT NULL,  -- user_id
    marked_at TEXT NOT NULL,
    notes TEXT,
    FOREIGN KEY (student_id) REFERENCES students(id),
    FOREIGN KEY (marked_by) REFERENCES users(id),
    UNIQUE(student_id, date)
);
```

### 5.2 Mark Attendance
**Route:** `GET /t/{slug}/attendance/mark`

**Features:**
- Select date (default: today)
- Select grade/class
- Quick mark all (Present/Absent)
- Individual status for each student
- Save with one click

**UI:**
```
Date: [2024-01-15]  Grade: [10 ▼]

┌─────────────────────────────────────┐
│ All: [Present] [Absent]            │
├─────────────────────────────────────┤
│ ☑ John Doe      [Present ▼]       │
│ ☑ Jane Smith    [Present ▼]       │
│ ☐ Bob Johnson   [Absent  ▼]       │
└─────────────────────────────────────┘
[Save Attendance]
```

### 5.3 Attendance Report
**Route:** `GET /t/{slug}/attendance/report`

**Features:**
- Date range selection
- Filter by student, grade, class
- Export to CSV/PDF
- Attendance percentage
- Charts and graphs

---

## Phase 6: Profile & Settings ⚙️

### Priority: LOW
User profile management and system settings.

### 6.1 User Profile
**Route:** `GET /t/{slug}/profile`

**Features:**
- View/edit own profile
- Change password
- View active sessions
- Logout from other devices

### 6.2 Change Password
**Route:** `POST /t/{slug}/profile/password`

**Features:**
- Current password verification
- New password strength indicator
- Logout all other sessions option
- Email notification

### 6.3 Tenant Settings (Admin)
**Route:** `GET /t/{slug}/admin/settings`

**Features:**
- Update tenant info (name, contact)
- Upload logo
- Configure academic year
- System preferences

---

## Phase 7: Advanced Features 🚀

### Priority: LOW (Future Enhancements)

### 7.1 Classes/Sections
- Create classes (e.g., "Grade 10 - Section A")
- Assign students to classes
- Assign teachers to classes

### 7.2 Subjects & Grades
- Define subjects
- Record grades/marks
- Generate report cards

### 7.3 Timetable
- Create class schedules
- Teacher timetables
- Student timetables

### 7.4 Fee Management
- Fee structure
- Collect payments
- Generate invoices
- Payment history

### 7.5 Notifications
- Email notifications
- SMS alerts (Twilio)
- In-app notifications
- Parent portal access

### 7.6 Reports
- Student progress reports
- Attendance reports
- Financial reports
- Custom report builder

---

## Technical Improvements

### Code Quality
- [ ] Add unit tests (critical paths)
- [ ] Add integration tests (auth flow)
- [ ] Error handling improvements
- [ ] Logging system (tracing crate)
- [ ] API documentation (Swagger/OpenAPI)

### Performance
- [ ] Database indexes optimization
- [ ] Query optimization
- [ ] Caching layer (optional)
- [ ] Connection pool tuning

### Security
- [ ] Rate limiting (login attempts)
- [ ] CSRF token validation
- [ ] Input sanitization
- [ ] SQL injection prevention audit
- [ ] XSS prevention audit
- [ ] HTTPS enforcement (production)
- [ ] Security headers

### DevOps
- [ ] Docker containerization
- [ ] CI/CD pipeline
- [ ] Database backups
- [ ] Monitoring & alerts
- [ ] Log aggregation

---

## Recommended Implementation Order

### Week 1-2: Core Student Management
1. Expand students table schema
2. List students page
3. Add student form
4. View student profile
5. Edit student
6. Delete student (soft delete)

### Week 3: User Management
1. User list (admin)
2. Create user
3. Edit user
4. User activation/deactivation

### Week 4: Session Management
1. Active sessions dashboard
2. Force logout functionality
3. Session limits
4. Login history

### Week 5-6: Attendance System
1. Attendance table schema
2. Mark attendance page
3. Attendance reports
4. Attendance statistics

### Week 7-8: Dashboard & Settings
1. Enhanced dashboard with real stats
2. User profile management
3. Change password
4. Tenant settings

### Beyond: Advanced Features
- Classes & sections
- Grades & subjects
- Timetable
- Fee management
- Notifications
- Advanced reports

---

## Quick Start for Next Session

### To Continue Development:

1. **Start with Student List:**
```bash
# Create new file
touch server/crates/student/src/list_students.rs

# Add to lib.rs
mod list_students;
```

2. **Expand Database Schema:**
```sql
-- Run against tenant database
sqlite3 tenant_demo-school.db

-- Add columns to students table
-- (see Phase 1.1 for full schema)
```

3. **Refer to Hotwire Patterns:**
- See `HOTWIRE_REFERENCE.md` for implementation patterns
- Use Turbo Frames for forms
- Use Turbo Streams for updates

4. **Follow Existing Patterns:**
- Feature-based files (one feature per file)
- DB → Logic → Templates → Handlers → Routes
- Use RequireAuth for protected routes
- Bootstrap 5 UI with icons

---

## Resources & Documentation

### Available Docs
- `HOTWIRE_REFERENCE.md` - Turbo implementation patterns
- `CUSTOM_SESSION_IMPLEMENTATION.md` - Session system guide
- `TENANT_ONBOARDING_GUIDE.md` - Tenant system guide
- `AUTH_SYSTEM_SUMMARY.md` - Authentication details

### Key Files to Know
- `server/crates/student/src/` - Student features
- `server/crates/auth/src/` - Authentication
- `server/crates/tenant/src/` - Tenant management
- `server/crates/shared/src/` - Shared utilities

### Database Files
- `master.db` - Tenant registry
- `tenant_{slug}.db` - Per-tenant data
- `SEED_ADMIN.sql` - Create test users

---

## Summary

**Current State:** ✅ Foundation Complete
- Multi-tenant system working
- Custom sessions implemented
- Authentication ready
- Modern UI in place

**Next Priority:** 🎓 Student Management
- This is the core of the ERP
- Start with list, add, view, edit
- Use Hotwire for smooth UX

**Timeline:** 
- Core features: 6-8 weeks
- Advanced features: 2-3 months
- Production ready: 3-4 months

**You have a solid foundation!** The hard parts (multi-tenancy, auth, sessions) are done. Now you can focus on building features! 🚀

---

**Good luck with your next session!** 🎉
