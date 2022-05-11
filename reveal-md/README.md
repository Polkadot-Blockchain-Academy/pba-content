# PBA [`reveal-md`](https://github.com/webpro/reveal-md) template

## Install

Only dependency is `reveal-md`. One of the two commands will work:

```sh
# If you want it only locally
npm i
# Globally
npm npm install --global reveal-md
```

## Live Server

This will watch for local file changes:

```sh
npm start
```

## Build & Deploy

This overwrites the `docs` folder with a static site of all `.md` files in `SLIDES/*`.
This can be used in github pages. Push any changes in this folder to `main` to have github pages deploy it live:

```sh
npm run build
```

> A live site is available [here](https://nukemandan.github.io/pba-template-reveal-md/) based on the `main` branch [here](https://github.com/NukeManDan/pba-template-reveal-md/)

## Custom Theme & CSS

For a full theme, [use one of these](https://github.com/hakimel/reveal.js/tree/master/css/theme/source) as a base. Set it in [reveal.json](reveal.json).
(Or [make your own theme](https://github.com/hakimel/reveal.js/tree/master/css/theme#creating-a-theme), based on these)

For simple css build on top of a theme, [use a css file](https://github.com/webpro/reveal-md#custom-css) set in [reveal-md.json](reveal-md.json)

## TODO

- [ ] set a ` --listing-template <filename> Template file for listing` used in the top level page for all slide decks (eventually replace with a popper nav in a more complete mono repo of materials for the academy)
- [ ] Fix background
- [ ] Get 2 colum working
- [ ] favicon.ico working


## Inspiration

- https://teaching.mrsharky.com/sdsu_fall_2020_lecture08.html#/7/4/8
-
