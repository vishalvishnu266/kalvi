# Hotwire Turbo Implementation Reference

This file documents how we implemented Hotwire Turbo in the test endpoints (now removed).
Use this as a reference for implementing real features.

## Key Concepts

### 1. Turbo Frames
Turbo Frames allow you to update parts of a page without full reload.

**HTML Structure:**
```html
<turbo-frame id="student_profile">
  <!-- Content here can be replaced -->
  <a href="/t/{slug}/student/{id}/edit">Edit</a>
</turbo-frame>
```

**Response from `/edit` endpoint:**
```html
<turbo-frame id="student_profile">
  <form action="/t/{slug}/student/{id}" method="post">
    <input name="name" value="...">
    <button type="submit">Save</button>
  </form>
</turbo-frame>
```

### 2. Turbo Streams
Turbo Streams allow multiple DOM updates in one response.

**Server Response:**
```rust
use maud::{html, PreEscaped};

fn render_turbo_stream_update(student: &Student) -> Markup {
    html! {
        // Replace the profile frame
        turbo-stream action="replace" target="student_profile" {
            template {
                turbo-frame id="student_profile" {
                    // Updated content
                }
            }
        }
        // Append a notification
        turbo-stream action="append" target="notifications" {
            template {
                div class="alert alert-success" {
                    "Student updated successfully!"
                }
            }
        }
    }
}
```

**Important:** Set correct content type:
```rust
use axum::response::Response;

let html = render_turbo_stream_update(&student).into_string();
Response::builder()
    .status(200)
    .header("Content-Type", "text/vnd.turbo-stream.html")
    .body(html.into())
    .unwrap()
```

### 3. Turbo Actions

Available actions for turbo-stream:
- `append` - Add content to end of target
- `prepend` - Add content to beginning of target
- `replace` - Replace entire target element
- `update` - Replace only inner HTML of target
- `remove` - Delete the target element
- `before` - Insert before the target
- `after` - Insert after the target

### 4. Required Script Tag

Include Turbo in your HTML:
```html
<script type="module">
  import hotwiredTurbo from 'https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/+esm'
</script>
```

## Implementation Pattern

### Step 1: Create Frame Template
```rust
fn render_item_frame(item: &Item) -> Markup {
    html! {
        turbo-frame id={"item_" (item.id)} {
            div class="item-display" {
                p { (item.name) }
                a href={"/t/{slug}/item/" (item.id) "/edit"} {
                    "Edit"
                }
            }
        }
    }
}
```

### Step 2: Create Edit Frame Template
```rust
fn render_item_edit_frame(item: &Item) -> Markup {
    html! {
        turbo-frame id={"item_" (item.id)} {
            form action={"/t/{slug}/item/" (item.id)} method="post" {
                input type="text" name="name" value=(item.name);
                button type="submit" { "Save" }
                a href={"/t/{slug}/item/" (item.id)} { "Cancel" }
            }
        }
    }
}
```

### Step 3: Create Update Handler with Turbo Stream
```rust
async fn update_item(
    Extension(pool): Extension<SqlitePool>,
    Path((tenant_slug, id)): Path<(String, i64)>,
    Form(form): Form<UpdateItemForm>,
) -> Response {
    // Update database
    let item = update_item_in_db(&pool, id, form).await.unwrap();
    
    // Return Turbo Stream response
    let html = html! {
        turbo-stream action="replace" target={"item_" (item.id)} {
            template {
                (render_item_frame(&item))
            }
        }
        turbo-stream action="append" target="flash_messages" {
            template {
                div class="alert alert-success alert-dismissible" {
                    "Item updated successfully!"
                    button type="button" 
                           class="btn-close" 
                           data-bs-dismiss="alert" {}
                }
            }
        }
    };
    
    Response::builder()
        .status(200)
        .header("Content-Type", "text/vnd.turbo-stream.html")
        .body(html.into_string().into())
        .unwrap()
}
```

### Step 4: Define Routes
```rust
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/t/{slug}/item/{id}", get(show_item_frame))
        .route("/t/{slug}/item/{id}/edit", get(edit_item_frame))
        .route("/t/{slug}/item/{id}", post(update_item))
}
```

## Benefits of Turbo

1. **Fast Navigation** - No full page reloads
2. **Progressive Enhancement** - Works without JavaScript (degrades gracefully)
3. **Simple Server Code** - Just return HTML, no JSON APIs needed
4. **Mobile Compatible** - Works great with Hotwire Native
5. **Less JavaScript** - No frontend framework required

## Common Patterns

### Pattern 1: Inline Editing
Click edit → form appears in place → submit → content updates

### Pattern 2: Modal Forms
Click add → modal opens with form → submit → list updates + modal closes

### Pattern 3: Live Updates
Form submits → multiple areas update (item + counter + notification)

### Pattern 4: Lazy Loading
Frame with src attribute loads content on demand

## Maud Helper Example

```rust
// Helper for turbo-stream wrapper
fn turbo_stream(action: &str, target: &str, content: Markup) -> Markup {
    html! {
        turbo-stream action=(action) target=(target) {
            template {
                (content)
            }
        }
    }
}

// Usage:
turbo_stream("replace", "item_1", render_item_frame(&item))
```

## Testing Tips

1. **Test without JavaScript first** - Should work as traditional form
2. **Test with Turbo enabled** - Should update without full reload
3. **Check Network tab** - Response should be `text/vnd.turbo-stream.html`
4. **Validate HTML structure** - turbo-frame IDs must match

## Resources

- Turbo Handbook: https://turbo.hotwired.dev/handbook/introduction
- Turbo Streams: https://turbo.hotwired.dev/handbook/streams
- Turbo Frames: https://turbo.hotwired.dev/handbook/frames

---

**Remember:** This is the pattern we used. Apply it to real features as needed!
