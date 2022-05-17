This guide is to help content contributors understand how to navigate this repository, format slides using `reveal.js` and run the repository locally.

> If this is your first time using `reveal.js`, have a look at [this official demo](https://revealjs.com/demo/#/2) to see what sort of things you can do with it. We'll keep things simple for now.

# Table of contents

- [Content organization](#content-organization)
- [Exercises, workshops and activities](#exercises-workshops-and-activities)
- [`reveal.js` basics](#-revealjs--basics)
  - [Slides](#slides)
  - [Using speaker notes](#using-speaker-notes)
  - [Transitions](#transitions)
  - [Code highlighting](#code-highlighting)
  - [Custom theme and CSS](#custom-theme-and-css)
  - [Presenting](#presenting)
- [Install and launch locally](#install-and-launch-locally)

## Content organization

The content is organized by module, where each module has a folder for a specific lecture, and every lecture has:

- A markdown file of the formatted slides (this is what `reveal.js` parses to render the final slides)
- A markdown file containing workshops and exercises for that lecture
- A folder containing the markdown files of the original notes and lesson plans for each lecture

Assuming you're in the `syllabus` directory, this structure looks like (where `*` is the name of the lecture):

```
syllabus/
├─ 1-example-module/
│  ├─ 1.1-*/
│  │  ├─ 1.1-Workshops_and_Activities/
│  │  │  ├─ 1.1-*_Activity.md
│  │  │  ├─ 1.1-*_Workshop.md
│  │  ├─ 1.1-*_Slides.md
│  ├─ example-module-lesson-plans/
│  |   ├─ 1.1-Lesson_Plan_Name_of_Lecture_1.md
├  |   ├─ 1.2-Lesson_Plan_Name_of_Lecture_2.md
├─ 2-another-example-module/
│  ├─ ...
```

When creating content for your slide, we recommend you:

1. Start with copying the "Core Ideas to Convey" section of the original lesson plan.
2. Build out the content slide by slide around those notes.
3. Add TODOs to write notes to yourself for adding diagrams or things to get back to later.

## Exercises, workshops and activities

Each lecture may have a set of exercises, workshops and/or activities:

* **Exercises**: these are short (5-10 minutes) exercises that are included as part of the slide deck and can be completing during the lecture.
* **Workshops**: these are step-by-step, guided in-class workshops, intended to be more like individual labs, whose worksheet would live in a separate folder with a separate file called `Name_of_Lecture_Workshop.md`.
* **Activities**: these are in-class activities too, however they are intended to be more like group activities, without the step-by-step guidance that a workshop would have. These live in a separate folder called `Workshops_and_Activities` in a separate file called `Name_of_Lecture_Activities.md`.

Note: not all lectures have workshops or activities.

## `reveal-md` basics

This section covers the basic things you need to know in order to get started with creating and customizing slides with `reveal.js`, and a tool built with it to allow for MarkDown only slides `reveal-md`.

## Install and launch all slides locally

The only dependency we need is `reveal-md`, you can install it with:

```sh
yarn
```

Then, running this command will open a new tab and watch for local file changes in real time:

```sh
yarn start
```

### Slides

When writing slides, separate each one using `---`.

Optionally, you can separate slides vertically using `----` between slides.

If several slides fit closely with some core topic being presented, it may be a good idea to stack those slides vertically.
For example, imagine the core topic was "Code Highlight & Transitions":

<img src="./assets/contributing-examples/vertical-slides.png" alt="vertical-slides" width="300"/>

### Using speaker notes

It's sometimes useful to have speaker notes for your slides.
This feature can be accessed when in presentation by pressing `s` when presenting (_you need to unblock popups to have the window open_).

To include notes for a slide, use the "Note" keyword inside that slide.
For example:

```md
Note: This is a note just for you. Set under a line in your slide starting with "`Note`:" all
subsequent lines are just seen in speaker view.
```

And here's an example of the result:

<img src="./assets/contributing-examples/speaker-notes-view.png" alt="vertical-slides" width="300"/>

### Transitions

To add transitions in a slide:

```md
_This will render only once the right or down arrow is pressed by presenter._

<!-- .element: class="fragment" data-fragment-index="2" -->
```

TODO: update this once we're more familiar with the css stuff.

### Code highlighting

To add a code snippet to your slide with highlighting and transitions:

```md
<pre><code style="font-size: 0.5em !important" data-trim data-noescape data-line-numbers="0|1|2|" class="rust">
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;

/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
>;
</pre></code>
```

You can use the pipes to select several lines too, for example: 1-14.

## Custom theme and CSS

TODO: once we have more details on the template for each lesson and how each `.md` links back to the `.html` page that renders it.

### Presenting

Once you've followed the set-up instructions and have the repository running locally, here are the basic ways to navigate through presenting your slides:

- Use `down/up` arrow keys to navigate _vertical_ slides.
- Use `left/right` arrow keys to navigate horizontal slides.
- Press `Esc` or `o` to see an `overview` view that your arrow keys can navigate. This allows you to click a slide to open it).
- Press `s` to open up speaker view.
