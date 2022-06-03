# Polkadot Blockchain Academy Contributors Guide

This guide is to help Academy contributors understand how all materials contained this repository are structured, how to interact with and modify them.
Multiple tools are provided for contributors to make slideshows, leader-guided workshops, and self-directed activities.

## Table of contents

- [Content organization](#content-organization)
- [Lesson slides with Reveal.js](#lesson-slides-with-revealjs)
- [Exercises, workshops and activities](#exercises-workshops-and-activities)
  - [Rust Jupyter notebooks with EvCxR](#rust-jupyter-notebooks-with-evcxr)
  - [Stand-alone Rust workshops and activities](#stand-alone-rust-workshops-and-activities)
  - [Conceptual workshops and exercises]

## Content organization

The content is organized by module, where each module has a folder for a specific lesson, and every lesson has:

- A markdown file of the `reveal-md` formatted **_lecture_** slides including **_exercises_** described within
- A markdown file outlining **_workshops_** and/or **_activities_** for that lesson.
- A folder containing the markdown files of the original lesson plan notes (from the Academy retreat)

The `syllabus` directory houses these, where `*` is the name of the lesson:

```
syllabus/
├─ 1-example-module/
│  ├─ 1.1-*/
│  │  ├─ 1.1-Workshops_and_Activities/
│  │  │  ├─ 1.1-*_Activity.md
│  │  │  ├─ 1.1-*_Workshop.md
│  │  ├─ 1.1-*_Slides.md
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
Please start with the [INSTRUCTIONS-HOW-TO-USE-REVEAL-MD-Slides.md](./reveal-md/INSTRUCTIONS-HOW-TO-USE-REVEAL-MD-Slides.md) slides to see what is possible with the slides features and some template slides.

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

### Run to view slides

Running this command will open a new browser tab and _watch for file changes_ (update on every time you save a file in this repo):

```sh
yarn start
```

</details>

---

**To see what reveal can do, please view the slides and their source for details:**

- The [how-to use reveal slides](https://paritytech.github.io/polkadot-blockchain-academy/content-templates/slides/0-how-to-use-reveal-slides.html) and the [source](./content-templates/slides/0-how-to-use-reveal-slides.md)
- The [copy & paste slide templates](https://paritytech.github.io/polkadot-blockchain-academy/content-templates/slides/1-copy-paste-reveal-template-slides.html) that are styled to use in your content and their [source](./content-templates/slides/1-copy-paste-reveal-template-slides.md)
- The [lesson template slides](https://paritytech.github.io/polkadot-blockchain-academy//content-templates/slides/lesson-template-slides.html) and their [source](/content-templates/slides/lesson-template-slides.md) that give the structure most lessons should use to start with.

---

### Editing `reveal-md` slides

**There is a [base slide template example](./content-templates/slides/lesson-template-slides.md) that is highly suggested to use as a base to start all lesson slides.**
This template can be used just copy and change slide by slide the content, and not worry with styling.
If you do need custom style, please just comment in the slides with a code comment that says as much:

```md
<img src="./assets/img/<module or shared>/some-tilted-thing.png" alt="tilted!">

<!-- TODO: I need this image to be rotated 45deg left -->
```

<details>
<summary>If you need more of an explanation on how to use `reveal-md`'s features, see below (click to toggle) </summary>

When writing slides, separate each one using `---`.

Optionally, you can separate slides vertically using `---v` between slides.

For distinct parts of the lecture, with some core topic being presented as the, it may be a good idea to stack those slides vertically for easier navigation.
For example, imagine the core topic was "Code Highlight & Transitions":

<img src="./assets/img/Shared/vertical-slides.png" alt="vertical-slides" width="300"/>

#### Using speaker notes

It's sometimes useful to have speaker notes for your slides.
This feature can be accessed when in presentation by pressing `s` when presenting (_you need to unblock popups to have the window open_).

To include notes for a slide, use the "Note" keyword inside that slide.
For example:

```md
Note: This is a note just for you.
Set under a line in your slide starting with "`Note`:" all
subsequent lines are just seen in speaker view.
```

And here's an example of the result:

<img src="./assets/img/Shared/speaker-notes-view.png" alt="vertical-slides" width="300"/>

#### Transitions

To add transitions in a slide:

```md
_This will render only once the right or down arrow is pressed by presenter._

<!-- .element: class="fragment" data-fragment-index="2" -->
```

TODO: update this once we're more familiar with the css stuff.

#### Code highlighting

You can add code snippets to your slides with line highlighting.
You can also animate to step through each highlight with `|` delimited sections of lines as a _fragment_:

````md
<!-- first fragment is line 0, meaning NO highlight -->
<!-- second fragment highlights lines 1 and 13 -->
<!-- last highlight is the block of lines from 4 to 8 -->

```rust [0|1,13|4-8]
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```
````

### Custom theme and CSS

The [template](#editing-reveal-md-slides) can be used just copy and change slide by slide the content, and not worry with styling.

If you do need custom style, please just comment in the slides with a code comment that says as much:

```md
<img src="../assets/img/place-holder/some-image-tilted.png" alt="tilted!">

<!-- TODO: I need this image to be rotated 45deg left -->
```

#### Presenting

Once you've followed the set-up instructions and have the repository running locally, here are the basic ways to navigate through presenting your slides:

- Use `down/up` arrow keys to navigate _vertical_ slides.
- Use `left/right` arrow keys to navigate horizontal slides.
- Press `Esc` or `o` to see an `overview` view that your arrow keys can navigate. This allows you to click a slide to open it).
- Press `s` to open up speaker view.
</details>

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

We _highly suggest_ that most activities involving simple Rust examples use the [EvCxR](#rust-jupyter-notebooks-with-evcxr) tooling for it's quite powerful features.
All materials needed for these that cannot exist in the code (like notebooks or custom source crates students need to download & use) should be included in the [assets/<the type>/<the module>](./assets/) directory.

Solutions should (when possible) be provided in _a separate branch of this repository_ such that material published on the `main` branch does not include these.
For example, skeleton code with code-comments providing instructions in a crate on `main` should have a `solution-modX-lessonY-*` branch where a completed reference that fulfills the requirements is available.

The following outline some suggested tools to use for these.

### Rust Jupyter notebooks with EvCxR

REPLs are a fantastic way to experiment with a language interactively.
[`evcxr_jupyter`](https://github.com/google/evcxr/tree/HEAD/evcxr_jupyter) uses the fantastic [Jupyter Notebook](https://jupyter.org/) tooling for interactive documents with a built-in REPL.

**Please watch this [Jupyter 101 demo video](https://youtu.be/HW29067qVWk?t=248) to get to know the basics before proceeding**

#### _Quick start_

1. Install [`evcxr_jupyter`](https://github.com/google/evcxr/tree/HEAD/evcxr_jupyter#installation)
2. Open the [evcxr_jupyter_pba_example.ipynb](/content-templates/evcxr_jupyter_pba_example.ipynb) with the tool of your choice:

- Easiest and likely to be suggested to students to _edit and use_ your notebooks (but you can edit too!) is the [VSCode plugin](https://github.com/Microsoft/vscode-jupyter).
  Install by searching for this plugin (`@id:ms-toolsai.jupyter`) in the VSCode extensions menu.
  Once installed, as with `evcxr_jupyter` installed, you can select the Rust kernel and start interacting with Rust-based notebooks like the example.
- Best-in-class support is [Jupyter Lab](https://jupyterlab.readthedocs.io/en/stable/) over the vanilla notebooks tooling and VSCode.
  Although most instructors and students will not need this.
  Installation [described here](https://jupyter.org/install).

> Note that sadly `rust-analyzer` [does not work with notebooks](https://github.com/rust-lang/rust-analyzer/issues/5141) at this time.
> Thus student's will have a harder time using unfamiliar crate's features (they are all new to rust).
> Please make any expected work in notebooks relatively simple with respect to crate's features and reference in code comments what API docs specifically are critical to use.

#### EvCxR Templates

1. Please start with the [tour notebook](./content-templates/jupyter-notebooks/evcxr_jupyter_tour.ipynb) to see what EvCxR can do!
2. Then see the [activity notebook template](./content-templates/jupyter-notebooks/activity-template.ipynb) that you are encouraged to use as a base for all activities.
3. Finally see the [substrate example notebook](./content-templates/jupyter-notebooks/substrate_example.ipynb) that demonstrates using substrate crates.

### Stand-alone Rust workshops and activities

For non-trivial Rust work, it's best to use a full IDE and cargo properly, over the REPL examples discussed above.
For these, please create well documented crates that stand alone for each workshop or activity.

** https://github.com/rust-lang/rustlings is a fantastic place to draw inspiration from.**
While the full CLI tool to make things interactive ins't required, all the [example modules](https://github.com/rust-lang/rustlings/tree/main/exercises) are!

**Please place stand-alone crates into the [./assets/Materials-Downloads/<the correct module>/<source>](./assets/Materials-Downloads/) directory for distribution to students.**
These can be referenced and then linked to from any slides for them to download or use in an online IDE.

Please make **a new branch in this repo** to store the solutions for your workshops and activities so that we can reference them, but will not be generally available on the deployed resources that students can access.

#### Local IDE

It is very likely that all students will want to run your code locally.
We suggest that most users will use VSCode as it's most all-around featureful for academy work more than just Rust itself.

**NOTE: The build times for you project need to be taken into account!**
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
- [Substrate Playground](https://docs.substrate.io/playground/) - This is a Parity hosted very powerful build machine that anyone can access using a pre-compiled docker image of the base [substrate node template](https://github.com/substrate-developer-hub/substrate-node-template) to build on top of in an online VSCode-like IDE.
  The session is limited to 2 hours per user, before all data is cleared. There is no simple way to capture your work though, so this is only to be used in simple examples of substrate to iterate very quickly.
  _Note that it is unclear if 50+ students can access this at the same time, thus please let the TAs know if you plan to use this first!_

### Conceptual workshops and activities

While _most_ work students are doing should highlight _practical applications_ of the concepts, sometimes code isn't the best way to engage.
For non-code based work, please see the [workshop outline template](content-templates/workshop-outline-template.md) that structures what we should include in workshops or activities.
This should be included in the [./assets/Materials-Downloads/<the correct module>/<source>](./assets/Materials-Downloads/) directory for distribution to students.
