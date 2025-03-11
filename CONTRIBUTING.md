# Polkadot Blockchain Academy Contributors Guide

This guide is to help Academy contributors understand how all materials contained this repository are structured, how to interact with and modify them.
Multiple tools are provided for contributors to make slideshows, leader-guided workshops, and self-directed activities.

## Table of contents

- [Polkadot Blockchain Academy Contributors Guide](#polkadot-blockchain-academy-contributors-guide)
  - [Table of contents](#table-of-contents)
  - [Content organization](#content-organization)
  - [Lesson slides with Reveal.js](#lesson-slides-with-revealjs)
    - [_Quick start_](#quick-start)
    - [Node.js](#nodejs)
    - [Yarn](#yarn)
      - [Using speaker notes](#using-speaker-notes)
      - [Presenting](#presenting)
    - [Formatting Markdown, TOML, JSON](#formatting-markdown-toml-json)
    - [Check embedded links](#check-embedded-links)
  - [Exercises, workshops and activities](#exercises-workshops-and-activities)
    - [Stand-alone Rust workshops and activities](#stand-alone-rust-workshops-and-activities)
      - [Local IDE](#local-ide)
      - [Online IDE](#online-ide)
    - [Conceptual workshops and activities](#conceptual-workshops-and-activities)

## Content organization

```
syllabus/
├─ 1-example-module/
│  ├─ 1.1-*/
│  │  ├─ 1.1-Workshops_and_Activities/
│  │  │  ├─ 1.1-*_Activity.md
│  │  │  ├─ 1.1-*_Workshop.md
│  │  ├─ 1.1-*_slides.md
│  ├─ example-module-lesson-plans/
│  |   ├─ 1.1-Lesson_Plan_*_1.md
├  |   ├─ 1.2-Lesson_Plan_*_2.md
├─ 2-another-example-module/
│  ├─ ...
```

When creating content for your lecture (and exercise) slides, we recommend you:

1. Start with copying the "Core Ideas to Convey" section of the original lesson plan.
1. Build out the content slide by slide around those notes.
1. Add TODOs as markdown comments (`<!-- this is a comment -->`) to write notes to yourself and to the TAs for adding diagrams or things to get back to later.
1. Place content for workshops and activities in the appropriate files, cross-referencing lecture and exercise content for reference to be build out around.

## Lesson slides with Reveal.js

**If this is your first time using `reveal.js`, we encourage you to explore [the official demo](https://revealjs.com/demo/#/2) to see what sort of things you can do with it!**
We are creating and customizing slides with [`reveal-md`](https://github.com/webpro/reveal-md): a tool built with `reveal.js` to allow for [Markdown](https://commonmark.org/help/) only slides, with a few extra syntax items to make _your slides look and feel awesome_ with very little effort.

### _Quick start_

Have `nvm` and `yarn` already installed? All you need to do is execute this from the top level directory of the Academy repo:

```sh
# Ensure you have the right node
nvm i
# For yarn 3, you need to enable some node features
corepack enable
# Install Dependencies
yarn
# Run a slide server watching for file changes
yarn start
```

This should open a new tab with a listing of all slide decks to choose from.

<details>
<summary>If you are missing node or yarn, please install them as described below. (click to toggle)</summary>

### Node.js

For all linux and mac users We suggest to use [nvm](https://github.com/nvm-sh/nvm#installing-and-updating) to install and manage multiple node versions.
With `nvm` installed, from the academy top level dir:

```sh
nvm install
```

This will install (if needed) and set the correct version to use for this project set in the `.nvmrc` file here.

If you choose to not use `nvm`, you need [node](https://nodejs.org/en/) of version greater than `16.10`.
It is likely your [package manager](https://nodejs.org/en/download/package-manager/#debian-and-ubuntu-based-linux-distributions) has this version for you.

### Yarn

Please see the [official guide](https://yarnpkg.com/getting-started/install) to install for yarn 3.
Likely all you need to do is:

```sh
corepack enable
```

The only dependencies we need for this project can now all be installed with:

```sh
yarn
```

</details>

---

#### Using speaker notes

It's sometimes useful to have speaker notes for your slides.
This feature can be accessed when in presentation by pressing `s` when presenting (_you need to unblock popups to have the window open_).

To include notes for a slide, use the "Note" keyword inside that slide.
For example:

```md
Notes:

This is a note just for you.
All these lines are just seen in speaker view only.
```

#### Presenting

Once you've followed the set-up instructions and have the repository running locally, here are the basic ways to navigate through presenting your slides:

- Use `down/up` arrow keys to navigate _vertical_ slides.
- Use `left/right` arrow keys to navigate horizontal slides.
- Press `Esc` or `o` to see an `overview` view that your arrow keys can navigate. This allows you to click a slide to open it).
- Press `s` to open up speaker view.

</details>

### Formatting Markdown, TOML, JSON

We enforce the use of a few formatters, the primary one being [Prettier](https://prettier.io/) that is included in the dev-dependencies for this repository.
In [package.json](./package.json) we include a handy tool to format all markdown docs easily:

```sh
# This will format all `syllabus/*.md` files
yarn lint:write
```

If (and only if) formatting _breaks_ markdown from rendering correctly, you may use `<!-- prettier-ignore -->` preceding a block in markdown to skip formatting like this:

````markdown
<!-- prettier-ignore -->
```html
<pba-cols>
<pba-col>

### What's up, yo?

</pba-col>
<pba-col>

- Yo
- Yo
- Yo

</pba-col>
</pba-cols>
```
````

The above ` ```html ` block will not be formatted.


### Remove notes from slides

If for any reason you want to remove the notes from the slides, you can run:
```
yarn run remove-notes <No-Module>
```
If for example you want to remove all notes from the Economics module you can run:
```
yarn run remove-notes 2-Economics
```

The script then will search all the files under `./syllabus/2-Economics` directory and remove all sequences that starts with `Notes:` and finishes with either `---` or `---v` (leaving that slide remaining).

### Check embedded links

To ensure all `*.md` contain no broken links within them, we have included a [simple link checker](https://github.com/tcort/markdown-link-check) you can run per module of content with:

```sh
# Link check a single file (or use regex to match)
yarn links <relative-link-to/the-top-working-dir/file.md>

# Link check all content files
yarn links syllabus/**/*.md

# This will check the files that match the pattern `syllabus/${MOD_NUMBER}*.md`.
yarn mod-links <Module Number>
```

The same tool is also run by our CI on all files for all pushes to all branches.
See [.github/workflows/link-check.yml](.github/workflows/link-check.yml) for details.
Both use the same [config file](.github/workflows/mlc_config.json).

> You can ignore the link check for a single line by post-fixing it with:
>
> `Some private or intentionally broken link. <!-- TODO Remove this check disable once ... --> <!-- markdown-link-check-disable-line -->`

## Exercises, workshops and activities

Each lecture may have a set of exercises, workshops and/or activities.
Not all lectures _must_ have workshops or activities... but almost all _should_.
**The academy is focused on _practical application_ of Web3 concepts we cover, more than simply understanding.**
Here is how we define these:

- **Exercises**: these are short (5-10 minutes) exercises that are included as part of the slide deck and will be completing during the lecture.
- **Workshops**: these are step-by-step, longer (30 min to 3 hours) guided in-class material.
  These are instructor lead, and hand-held to get everyone to the same result.
- **Activities**: these are self-directed assignments for individuals and/or small groups that do not "hand-hold" like workshops.
  Student's completed work is expected to have some variety and a canonical solutions should be produced to review when students submit to compare to.

Solutions should (when possible) be provided in _a separate branch of this repository_ such that material published on the `main` branch does not include these.
For example, skeleton code with code-comments providing instructions in a crate on `main` should have a `solution-modX-lessonY-*` branch where a completed reference that fulfills the requirements is available.

The following outline some suggested tools to use for these.

### Stand-alone Rust workshops and activities

For non-trivial Rust work, it's best to use a full IDE and cargo properly.
For these, please create well documented crates in their own repository that stand alone for each workshop or activity.
We will use github classrooms to give each student their own fork of such repositories.

#### Local IDE

It is very likely that all students will want to run your code locally.
We suggest that most users will use VSCode as it's most all-around featureful for academy work more than just Rust itself.

**Notice: The build times for you project need to be taken into account!**
Please time on your build machine as a reference and report this to the TAs so we can all plan around the rough timeline to have students start to build things.

- We **highly suggest [`sccache`](https://github.com/mozilla/sccache)** that will enable faster builds for almost all academy students! If you want to use it globally, you need to add this with the right path to your `~/.cargo/config.toml` file:

  ```toml
  [build]
  rustc-wrapper = "<path to where>/.cargo/bin/sccache"
  ```

  Use `which sccache` to find the path.

- To get more power out of `sccache` and maybe overall faster linking, install and use the [`lld` linker](https://lld.llvm.org/) and while notebooks using EvCxR use this by default if detected, if you want to use it globally, see [this post](https://stackoverflow.com/questions/57812916/how-do-i-change-the-default-rustc-cargo-linker) on how to enable it. You need to add something like this to your `~/.cargo/config.toml` file:

  ```toml
  [target.x86_64-unknown-linux-gnu]
  rustflags = [
    "-C", "link-arg=-fuse-ld=lld",
  ]
  ```

#### Online IDE

There are some great (but limited) options for anyone lacking the ability to do things locally, or real-time collaboration is needed.

- [REPL.it](https://replit.com/languages/rust) - This online IDE includes a low-powered machine to compile Rust on, but provides _real-time collaboration_ where multiple users can follow-the-leader and edit simultaneously.
  It also features github integrations to save and share progress.

### Conceptual workshops and activities

While _most_ work students are doing should highlight _practical applications_ of the concepts, sometimes code isn't the best way to engage.
