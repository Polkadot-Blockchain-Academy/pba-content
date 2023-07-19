---
title: How-to use Reveal.js
description: How to use reveal.js
duration: 5 minuets
---

# How-to use Reveal.js

These slides are built with [reveal.js](https://revealjs.com/).

Please first view the [CONTRIBUTING.md](../CONTRIBUTING.md) guide on how to use these slides.

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

# How to use Polkadot icons

All Polkadot icons exist under the directory `/assets/icons/polkadot/{type}/{icon_name}.svg`, where:

- `type` can be one of: `line`, `solid`, `2color`;
- `icon_name` is the name of the icon (e.g. `Alice`, `Oracles` etc);

> You can find the icons' relative names by browsing to [Polkadot icons set - https://icons.polkadot.network](https://icons.polkadot.network)

---v

In order to effectively use those icons in their default form (meaning `line` in white, `solid` in Polkadot pink and `2color` in a combination of white and Polkadot pink) you can simply do:

```html
<img src="./assets/icons/polkadot/2color/Ecosystem.svg" />
<img src="./assets/icons/polkadot/line/Ecosystem.svg" />
<img src="./assets/icons/polkadot/solid/Ecosystem.svg" />
```

<br/> and the outcomes, accordingly, will be:

<img src="../assets/icons/polkadot/2color/Ecosystem.svg" /><br />
<img src="../assets/icons/polkadot/line/Ecosystem.svg" /><br />
<img src="../assets/icons/polkadot/solid/Ecosystem.svg" />

---v

For using custom colors and/or css attributes, you will need to add the `style` attribute on the `img` tag with the needed changes:

```html
<img
  style="filter:
    brightness(0)
    saturate(100%)
    invert(37%)
    sepia(83%)
    saturate(2704%)
    hue-rotate(132deg)
    brightness(93%)
    contrast(84%);"
  src="./assets/icons/polkadot/2color/Ecosystem.svg"
/>
```

<br/> and the outcome, will be:

<img
  style="filter: brightness(0) saturate(100%) invert(37%) sepia(83%) saturate(2704%) hue-rotate(132deg) brightness(93%) contrast(84%);"
  src="../assets/icons/polkadot/2color/Ecosystem.svg"
/>

> For altering color the `filter` arg should be used[ a filter generator](https://angel-rs.github.io/css-color-filter-generator/) can be used for extracting code from hex color)

---

# More help needed?

👋 Please reach out to the academy content & docs team on element for support!
