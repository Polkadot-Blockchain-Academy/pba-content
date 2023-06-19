---
title: How-to use Reveal.js
description: How to use reveal.js
duration: 5 minuets
---

# How-to use Reveal.js

These slides are built with [reveal.js](https://revealjs.com/).

Please first view the [CONTRIBUTING.md](../../CONTRIBUTING.md) guide on how to use these slides.

These slides serve as a feature demo of reveal for you! 🎉

---

## What are we going to see:

- How to use Reveal.js Features

  - Useful `reveal.js` tips
  - Speaker Notes

- [Template Slides](#template-slides)

---

## How to use Reveal.js Features

_Press the `down/up` keys to navigate \_vertical_ slides\_

Try doing down a slide.

<!-- .element: class="fragment" -->

---v

### Use the keybindings!

- **Overview mode**: “O” to see a birds-eye view of your presentation, “ESC” to return to the highlighted slide (you can quickly navigate with arrows)

- **Full-screen**: “F”, “ESC” to exit full-screen mode

- **Speaker mode**: “S” it synchronizes 2 windows: one with the presentation, and another with a timer and all speaker notes!

- **Zoom-in**: ALT+click make the view zoom at the position of your mouse’s pointer; very useful to look closely at a picture or chart surrounded by too much bullet points.

---v

## Speaker Notes & Viewer

_Press the `s` key to bring up a popup window with speaker view_

**You need to unblock popups to have the window open**

Notes:
This is a note just for you. Set under a line in your slide starting with "`Note`:" all
subsequent lines are just seen in speaker view.

---

# Template slides

😎 Copy&paste development with the [lesson template slides](./1-TEMPLATE_lecture_slides.md)

---

# How to use Mermaid Diagrams

[Mermaid](https://mermaid.js.org) lets you create diagrams and visualizations using text and code.

It is a JavaScript based diagramming and charting tool that renders Markdown-inspired text definitions to create and modify diagrams dynamically.

First of all lets see some examples of diagrams that Mermaid can show with its integration with revealJS;

---v

### [A Flowchart](https://mermaid.js.org/syntax/flowchart.html)

  <div class="mermaid">
    <pre>
      %%{init: {'theme': 'dark', 'themeVariables': { 'darkMode': true }}}%%
      flowchart TD
        A[Start] --> B{Is it?};
        B -- Yes --> C[OK];
        C --> D[Rethink];
        D --> B;
        B -- No ----> E[End];
    </pre>
  </div>

---v

### And its code

```html
<div class="mermaid">
  <pre>
    %%{init: {'theme': 'dark', 'themeVariables': { 'darkMode': true }}}%%
    flowchart TD
      A[Start] --> B{Is it?};
      B -- Yes --> C[OK];
      C --> D[Rethink];
      D --> B;
      B -- No ----> E[End];
  </pre>
</div>
```

---v

### Entity relationship diagram

<div class="mermaid">
  <pre>
    erDiagram
    Node ||--o{ Wallet : places_order
    Wallet ||--|{ Account : owner
    Node }|..|{ Some-IP : uses
  </pre>
</div>

---v

### And its code

```html
<div class="mermaid">
  <pre>
    erDiagram
    Node ||--o{ Wallet : places_order
    Wallet ||--|{ Account : owner
    Node }|..|{ Some-IP : uses
  </pre>
</div>
```

---v

### Useful links

- [Mermaid Syntax](https://mermaid.js.org/syntax/flowchart.html)
- [Mermaid Live Editor with examples](https://mermaid.live/)

---

# More help needed?

👋 Please reach out to the academy content & docs team on element for support!
