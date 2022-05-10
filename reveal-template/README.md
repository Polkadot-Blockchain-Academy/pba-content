# [`reveal-md`](https://github.com/webpro/reveal-md) template

## Install

Only dependency is `reveal-md`. One of the two commands will work:

```sh
# Navigate to this directory 
cd reveal-template
# If you want it only locally
npm i
# Globally
npm install --global reveal-md
```

## Live Server

This will watch for local file changes:

```sh
npm start
```

## Build & Deploy
 
This can be used to build a static site to use anywhere:

```sh
npm run build
```

## Custom Theme & CSS

For a full theme, [use one of these](https://github.com/hakimel/reveal.js/tree/master/css/theme/source) as a base. Set it in [reveal.json](reveal.json).
(Or [make your own theme](https://github.com/hakimel/reveal.js/tree/master/css/theme#creating-a-theme), based on these)

For simple css build on top of a theme, [use a css file](https://github.com/webpro/reveal-md#custom-css) set in [reveal-md.json](reveal-md.json)

## TODO

- [ ] set a ` --listing-template <filename> Template file for listing` used in the top level page for all slide decks (eventually replace with a popper nav in a more complete mono repo of materials for the academy)
- [ ] Fix background, not a static image, but elements that live over the rest of the page
     - [ ]  build not finding image
- [ ] Get 2 colum working
- [ ] favicon.ico working
- [ ] more plugins (from a custom template per slide type {math vs no math} is better likely) https://github.com/webpro/reveal-md/issues/102#issuecomment-692494366 -- possibly math from latex https://revealjs.com/math/
- [ ] custom div to wrap slides of particular format, like a code view 

## Inspiration

- https://teaching.mrsharky.com/sdsu_fall_2020_lecture08.html#/7/4/8
- https://lacourt.dev/2019/03/12
- https://github.com/hakimel/reveal.js/wiki/Articles-&-Tutorials
