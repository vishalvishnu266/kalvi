# 🚀 Hotwire Turbo Demo - Complete Example

This is a **working, interactive demo** that shows you exactly how Hotwire Turbo works in your Rust application!

## 📍 What's Been Added

A new feature file: `crates/student/src/edit_student_turbo.rs`

This single file contains a complete demonstration of:
- ✅ **Turbo Frames** - Partial page updates
- ✅ **Turbo Streams** - Multiple simultaneous updates
- ✅ **Progressive Enhancement** - Works with or without JavaScript

## 🎯 How to Run

1. Start your server:
   ```bash
   cargo run
   ```

2. Visit the demo page:
   ```
   http://localhost:3000/turbo-demo/student/1
   ```

3. Open your browser's **Developer Tools** → **Network tab**

4. Watch the magic happen!

## 🎬 What You'll See

### The Demo Page Shows:

```
┌─────────────────────────────────────────────────────┐
│  🚀 Hotwire Turbo Demo                              │
│  Watch the page update WITHOUT full reload!         │
└─────────────────────────────────────────────────────┘

┌──────────────────────┐  ┌──────────────────────┐
│ 📦 TURBO FRAME #1    │  │ 🔄 TURBO STREAM      │
│                      │  │ TARGET               │
│ ┌──────────────────┐ │  │                      │
│ │ Student Profile  │ │  │ ✅ Ready to update   │
│ │ ID: 1            │ │  │                      │
│ │ Name: John Doe   │ │  └──────────────────────┘
│ │                  │ │
│ │ [Edit] [Back]    │ │  ┌──────────────────────┐
│ └──────────────────┘ │  │ 📊 ANOTHER STREAM    │
└──────────────────────┘  │ TARGET               │
                          │                      │
                          │ ⏰ Not modified yet  │
                          └──────────────────────┘
```

## 🎮 Interactive Demo Steps

### Step 1: Click "Edit"
- **What happens**: Only the left frame updates (no page reload!)
- **Check Network tab**: You'll see a request for just the edit form
- **The form appears** in place of the profile

### Step 2: Change the Name
- Type a new name in the input field

### Step 3: Click "Save"
- **What happens**: THREE things update at once!
  1. Left frame → Shows updated profile
  2. Top-right box → Shows update counter
  3. Bottom-right box → Shows "Last modified: Just now!"
- **Check Network tab**: 
  - No full page reload!
  - Server returns a `turbo-stream` response
  - Only the changed parts re-render

### Step 4: Try Without JavaScript
1. Disable JavaScript in DevTools
2. Click "Edit" → Full page loads (but still works!)
3. Save → Full page refresh (but data still updates!)

**This is Progressive Enhancement!** 🎉

## 💡 How It Works

### 1. Turbo Frames (Partial Updates)

```rust
// In the HTML template:
turbo-frame id="student_profile" {
    div.card {
        // Profile content here
        a href="/turbo-demo/student/1/edit" {
            "Edit"
        }
    }
}
```

When you click "Edit":
- Turbo intercepts the click
- Fetches `/turbo-demo/student/1/edit`
- Replaces **only** the `student_profile` frame
- Rest of page stays unchanged!

### 2. Turbo Streams (Multiple Updates)

```rust
// Server returns this after form submit:
fn render_turbo_stream_update(student: &Student) -> Markup {
    html! {
        // Update frame 1
        turbo-stream action="replace" target="student_profile" {
            template { /* new profile HTML */ }
        }
        
        // Update frame 2
        turbo-stream action="update" target="update-counter" {
            template { /* counter HTML */ }
        }
        
        // Update frame 3
        turbo-stream action="update" target="last-modified" {
            template { /* timestamp HTML */ }
        }
    }
}
```

All three updates happen **simultaneously** with one request!

### 3. The Response Type Matters

```rust
// When returning Turbo Stream, set this header:
(
    StatusCode::OK,
    [("Content-Type", "text/vnd.turbo-stream.html")],
    Html(render_turbo_stream_update(&student).into_string())
)
```

Turbo sees this content type and applies all the stream actions.

## 🔍 Code Structure

The demo file follows your feature-based architecture:

```rust
// edit_student_turbo.rs

// Models & DTOs
struct UpdateStudentForm { ... }

// Database Layer
async fn update_student_name(...) { ... }

// Templates (Maud)
fn render_demo_page(...) { ... }
fn render_student_profile_frame(...) { ... }
fn render_student_edit_frame(...) { ... }
fn render_turbo_stream_update(...) { ... }

// HTTP Handlers
async fn show_demo_page(...) { ... }
async fn edit_student_frame(...) { ... }
async fn update_student(...) { ... }

// Routes
pub fn routes() -> Router<AppState> { ... }
```

**Everything in one file!** Easy to understand, easy to modify.

## 📊 Network Request Flow

### Traditional Way (Full Page Reload):
```
Click Edit → GET /edit → Entire HTML page → Browser renders everything
Save → POST /save → Entire HTML page → Browser renders everything
```

### With Turbo Frames:
```
Click Edit → GET /edit → Just the edit form HTML → Swap only the frame
```

### With Turbo Streams:
```
Save → POST /save → Turbo Stream response → Update 3 parts independently
```

## 🎨 Turbo Stream Actions

The demo uses these actions:

| Action | What It Does | Example |
|--------|-------------|---------|
| `replace` | Replaces entire element including wrapper | Profile card gets completely replaced |
| `update` | Replaces inner HTML only | Counter content updates |
| `append` | Adds to end of element | Not used in demo, but useful for lists |
| `prepend` | Adds to start of element | Not used in demo, but useful for chat |
| `remove` | Removes element | Not used in demo, but useful for delete |

## 🚀 Benefits You Get

1. **Fast**: No full page reloads
2. **Smooth**: Updates feel instant
3. **Simple**: Just HTML from server, no complex JavaScript
4. **Resilient**: Works without JavaScript (progressive enhancement)
5. **SEO-Friendly**: Server-rendered HTML
6. **Easy to Debug**: View source shows real HTML

## 🛠️ Extending the Demo

### Add More Turbo Frames

Want to make the update counter editable too?

```rust
turbo-frame id="update_settings" {
    // Some settings form
}
```

### Add Turbo Stream Animations

Add CSS for smooth transitions:

```css
turbo-frame {
    transition: opacity 0.2s;
}

turbo-frame[busy] {
    opacity: 0.6;
}
```

### Add Real-Time Updates

Combine with Server-Sent Events or WebSockets:

```rust
// Stream updates to all connected clients
turbo-stream action="append" target="notifications" {
    template {
        div.alert { "New student added!" }
    }
}
```

## 🎓 Learning Points

After exploring this demo, you understand:

✅ **Turbo Frames** isolate parts of the page for independent updates
✅ **Turbo Streams** can update multiple parts in one response
✅ **Content-Type matters** - `text/vnd.turbo-stream.html` triggers streams
✅ **Same ID** on request and response makes frames work
✅ **Progressive enhancement** means it works without JS
✅ **Server-side rendering** with Maud works perfectly with Turbo

## 📝 Quick Reference

### Turbo Frame Template
```rust
turbo-frame id="unique_id" {
    // content
    a href="/some/path" { "Link" }  // Navigates within frame
}
```

### Turbo Stream Template
```rust
turbo-stream action="replace" target="unique_id" {
    template {
        // new HTML content
    }
}
```

### Response Header
```rust
[("Content-Type", "text/vnd.turbo-stream.html")]
```

## 🎉 Next Steps

1. **Run the demo** and play with it
2. **Open DevTools** and watch the network requests
3. **Modify the templates** in `edit_student_turbo.rs`
4. **Add your own Turbo Frames** to other features
5. **Experiment with Turbo Streams** for different actions

## 💬 Common Questions

**Q: Do I need to use Turbo everywhere?**
A: No! Mix and match. Some pages can use Turbo, others don't have to.

**Q: What if JavaScript is disabled?**
A: Everything still works! Forms submit normally, links navigate normally.

**Q: Can I use this with HTMX?**
A: Yes, but pick one. They solve similar problems. Turbo is simpler for most cases.

**Q: Does this work with the shared layout?**
A: Yes! You can use Turbo Frames inside the shared layout from `layout.rs`.

---

**Enjoy building fast, modern web apps with Hotwire Turbo!** 🚀
