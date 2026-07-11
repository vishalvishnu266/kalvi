# Hotwire Guide

## Setup

`templates/base.html` imports Turbo 8 as an ES module from jsDelivr:

```html
<script type="module">
  import * as Turbo from 'https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/+esm';
  window.Turbo = Turbo;
</script>
```

Once loaded, Turbo Drive intercepts link clicks and form submissions so navigations don't do full page reloads.

## Form submissions

All our POST handlers respond with `Redirect::to("...")` (which is `303 See Other`). Turbo follows the redirect via `GET` — that's the recommended pattern.

## Turbo Frames (planned use)

Wrap a section in `<turbo-frame id="foo">…</turbo-frame>`. Any link or form inside that frame targeting a URL whose response contains a `<turbo-frame id="foo">` will swap the frame's contents in place.

Example:

```html
<turbo-frame id="student-list">
  {% include "students/list.html" %}
</turbo-frame>
```

## Turbo Streams (planned use)

For updating multiple parts of the page from a single response, set the response `Content-Type` to `text/vnd.turbo-stream.html` and return one or more `<turbo-stream>` fragments:

```html
<turbo-stream action="append" target="messages">
  <template><li>New message</li></template>
</turbo-stream>
```

Actions: `append`, `prepend`, `replace`, `update`, `remove`, `before`, `after`.

## Askama helper

Common Turbo Stream responses can be modelled as small Askama templates so they compose the same way pages do.
