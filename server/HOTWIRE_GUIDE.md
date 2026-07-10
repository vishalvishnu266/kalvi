# Hotwire/Turbo Compatibility Guide

## ✅ YES! This Architecture is PERFECT for Hotwire!

This feature-based architecture works **exceptionally well** with Hotwire (Turbo + Stimulus). In fact, it's arguably **better** than traditional architectures for Hotwire.

## 🎯 Why This Architecture + Hotwire = 💯

### 1. **Server-Side HTML Rendering** ✅

Hotwire requires server-rendered HTML, which is exactly what we do with Maud!

```rust
// Each feature renders complete HTML
fn render_student_profile(student: &Student) -> Markup {
    html! {
        div.card {
            h2 { (student.name) }
            // Turbo can replace this entire div!
        }
    }
}
```

### 2. **One Feature = One Turbo Frame** ✅

Each feature file naturally maps to Turbo Frames/Streams:

```
student/view_student.rs    → <turbo-frame id="student-profile">
student/create_student.rs  → <turbo-frame id="student-form">
student/list_students.rs   → <turbo-frame id="student-list">
```

### 3. **Partial Rendering Built-In** ✅

You can easily add partial templates for Turbo Streams:

```rust
// In view_student.rs

// Full page template
fn render_student_profile_page(student: &Student) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head { /* ... */ }
            body {
                (render_student_profile_partial(student))
            }
        }
    }
}

// Partial for Turbo Frame/Stream
fn render_student_profile_partial(student: &Student) -> Markup {
    html! {
        turbo-frame id="student-profile" {
            div.card {
                h2 { (student.name) }
                p { "ID: " (student.id) }
            }
        }
    }
}

// Handler returns appropriate template
async fn view_student_handler(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let student = get_student(&pool, id).await?;
    
    // Check if Turbo Frame request
    if headers.get("Turbo-Frame").is_some() {
        Html(render_student_profile_partial(&student).into_string())
    } else {
        Html(render_student_profile_page(&student).into_string())
    }
}
```

## 🚀 Hotwire Integration Example

### Step 1: Add Hotwire to Your Templates

```rust
// In view_student.rs

fn render_student_profile(student: &Student) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Student Profile" }
                
                // Bootstrap CSS
                link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css" 
                     rel="stylesheet";
                
                // Hotwire Turbo
                script type="module" {
                    "import * as Turbo from 'https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/+esm';"
                }
                
                // Stimulus (optional)
                script type="module" {
                    r#"
                    import { Application, Controller } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';
                    
                    window.Stimulus = Application.start();
                    
                    class StudentController extends Controller {
                        connect() {
                            console.log('Student controller connected!');
                        }
                    }
                    
                    Stimulus.register('student', StudentController);
                    "#
                }
            }
            body {
                // Turbo Frame wraps the content
                turbo-frame id="student-profile" {
                    div.container.mt-5 data-controller="student" {
                        div.card {
                            div.card-header {
                                h2 { "Student Profile" }
                            }
                            div.card-body {
                                p { strong { "Name: " } (student.name) }
                                
                                // This link will update only the Turbo Frame!
                                a.btn.btn-primary 
                                  href={ "/student/" (student.id) "/edit" } 
                                  data-turbo-frame="student-profile" {
                                    "Edit Student"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
```

### Step 2: Create Edit Form (Turbo Frame)

```rust
// In edit_student.rs

fn render_edit_form(student: &Student) -> Markup {
    html! {
        turbo-frame id="student-profile" {
            div.container.mt-5 {
                div.card {
                    div.card-header {
                        h2 { "Edit Student" }
                    }
                    div.card-body {
                        // Form will submit via Turbo!
                        form method="post" action={ "/student/" (student.id) } {
                            div.mb-3 {
                                label.form-label for="name" { "Name" }
                                input#name.form-control type="text" name="name" 
                                      value=(student.name) required;
                            }
                            button.btn.btn-primary type="submit" { "Save" }
                            a.btn.btn-secondary.ms-2 
                              href={ "/student/" (student.id) } 
                              data-turbo-frame="student-profile" {
                                "Cancel"
                            }
                        }
                    }
                }
            }
        }
    }
}
```

### Step 3: Handle Form Submission with Turbo Stream

```rust
// In edit_student.rs

async fn update_student_handler(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
    Form(req): Form<UpdateStudentRequest>,
) -> impl IntoResponse {
    match update_student(&pool, id, req).await {
        Ok(student) => {
            // Return Turbo Stream to replace the frame
            let html = html! {
                turbo-stream action="replace" target="student-profile" {
                    template {
                        (render_student_profile_partial(&student))
                    }
                }
            };
            
            (
                [("Content-Type", "text/vnd.turbo-stream.html")],
                Html(html.into_string())
            )
        }
        Err(_) => {
            // Return error Turbo Stream
            let html = html! {
                turbo-stream action="prepend" target="student-profile" {
                    template {
                        div.alert.alert-danger { "Error updating student" }
                    }
                }
            };
            
            (
                [("Content-Type", "text/vnd.turbo-stream.html")],
                Html(html.into_string())
            )
        }
    }
}
```

## 🎨 Turbo Streams for Real-Time Updates

```rust
// In list_students.rs

// Server-Sent Events endpoint for live updates
async fn student_updates_stream() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = // ... your update stream logic
    
    Sse::new(stream.map(|student| {
        Event::default()
            .event("student_updated")
            .data(html! {
                turbo-stream action="replace" target={ "student-" (student.id) } {
                    template {
                        (render_student_row(&student))
                    }
                }
            }.into_string())
    }))
}
```

## 📋 Feature File Structure with Hotwire

```rust
// student/src/view_student.rs

// ============================================================================
// TEMPLATES - Full Pages
// ============================================================================

fn render_student_profile_page(student: &Student) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head { (render_head()) }
            body { (render_student_profile_partial(student)) }
        }
    }
}

// ============================================================================
// TEMPLATES - Partials for Turbo Frames
// ============================================================================

fn render_student_profile_partial(student: &Student) -> Markup {
    html! {
        turbo-frame id="student-profile" {
            div.card {
                // Content that can be replaced by Turbo
            }
        }
    }
}

// ============================================================================
// TEMPLATES - Turbo Streams
// ============================================================================

fn render_student_update_stream(student: &Student) -> Markup {
    html! {
        turbo-stream action="replace" target="student-profile" {
            template {
                (render_student_profile_partial(student))
            }
        }
    }
}

// ============================================================================
// HANDLERS - Detect Turbo Requests
// ============================================================================

async fn view_student_handler(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let student = get_student(&pool, id).await?;
    
    // Turbo Frame request?
    if headers.get("Turbo-Frame").is_some() {
        return Html(render_student_profile_partial(&student).into_string());
    }
    
    // Full page request
    Html(render_student_profile_page(&student).into_string())
}

async fn update_student_handler(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
    Form(req): Form<UpdateStudentRequest>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let student = update_student(&pool, id, req).await?;
    
    // Accept header includes turbo-stream?
    if headers
        .get("Accept")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.contains("turbo-stream"))
        .unwrap_or(false)
    {
        return (
            [("Content-Type", "text/vnd.turbo-stream.html")],
            Html(render_student_update_stream(&student).into_string())
        );
    }
    
    // Redirect for non-Turbo requests
    Redirect::to(&format!("/student/{}", id))
}
```

## 💡 Advantages of This Architecture + Hotwire

### ✅ Clear Feature Boundaries
Each Turbo Frame maps to a feature file:
- `view_student.rs` → `<turbo-frame id="student-profile">`
- `create_student.rs` → `<turbo-frame id="create-student-form">`
- `attendance.rs` → `<turbo-frame id="attendance-calendar">`

### ✅ Templates Stay Close to Logic
AI can see the full flow in one file:
- Form validation → Database update → Turbo Stream response
- No jumping between template files

### ✅ Easy to Add Interactivity
Add Stimulus controllers inline:

```rust
html! {
    div data-controller="student-search" {
        input type="text" 
              data-student-search-target="input"
              data-action="input->student-search#filter";
    }
    
    script {
        r#"
        Stimulus.register('student-search', class extends Controller {
            static targets = ['input'];
            
            filter() {
                // Filter logic
            }
        });
        "#
    }
}
```

### ✅ Progressive Enhancement
Full page loads work without JavaScript, Turbo enhances:

```rust
// Works with and without Turbo!
a href="/student/1" { "View Student" }

// Turbo intercepts and does SPA-like navigation
// Without Turbo: regular page load
```

## 🎯 Best Practices

### 1. **Separate Full Pages from Partials**

```rust
// Full page (initial load)
fn render_page(data: &Data) -> Markup { /* ... */ }

// Partial (Turbo updates)
fn render_partial(data: &Data) -> Markup { /* ... */ }

// Turbo Stream (real-time updates)
fn render_stream(data: &Data) -> Markup { /* ... */ }
```

### 2. **Use Data Attributes for Stimulus**

```rust
html! {
    div data-controller="form-validator"
        data-form-validator-url-value="/api/validate" {
        // Content
    }
}
```

### 3. **Keep JavaScript Minimal**

Most interactivity is handled by Turbo. Only use Stimulus for:
- Form validation
- Client-side filtering
- UI animations
- Third-party integrations

### 4. **One Feature = One Turbo Frame ID**

```rust
// student/view_student.rs → turbo-frame id="student-profile"
// student/edit_student.rs → turbo-frame id="student-profile" (same!)
// student/list_students.rs → turbo-frame id="student-list"
```

## 📚 Additional Resources

### Hotwire/Turbo Documentation
- https://turbo.hotwired.dev/
- https://stimulus.hotwired.dev/

### Example: Full CRUD with Hotwire

See `HOTWIRE_EXAMPLE.md` for a complete example of:
- List with search (Turbo Frames)
- Inline editing (Turbo Frames)
- Real-time updates (Turbo Streams)
- Optimistic UI (Stimulus)

## ✅ Summary

**This architecture is EXCELLENT for Hotwire because:**

1. ✅ Server-side HTML rendering (Maud) = Perfect for Turbo
2. ✅ One feature = One file = One Turbo Frame
3. ✅ Partials and full pages in same file = Easy to maintain
4. ✅ Templates close to handlers = AI-friendly
5. ✅ Bootstrap + Turbo = Beautiful, fast UI
6. ✅ No build step needed = Simple deployment

**Hotwire + This Architecture = The perfect combo for modern web apps!** 🚀

---

**Want to see a complete example? Check out HOTWIRE_EXAMPLE.md (to be created)**
