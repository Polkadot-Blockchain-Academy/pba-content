---
title: How to use Academy Resources # Also update the h1 header on the first slide to the same name
description: Describe your slides here
duration: 1 hour
instructors: ["some one", "another gal"]
teaching-assistants: ["some one", "another gal"]
---

# How to use

# Academy Resources

### _SME Developer Meta-lecture_

---

<div class="flex-container">
<div class="left text-right"> <!-- Gotcha: You Need an empty line to render MD inside <div> -->

<!-- TODO: add a good circularly cropped head shot of ou to the `assets/profile` folder  -->
<img style="width: 550px; float:right; margin-right:30px" src="../../assets/img/0-Shared/people/BUFFICORN_2521_ALT_smol.gif"/>

</div>
<div style="margin-top:130px" class="right text-left"> <!-- Gotcha: You Need an empty line to render MD inside <div> -->

### Dan Shields (NukeManDan)

<!-- .element: style="margin-bottom: -30px;" -->

#### _Developer Experience Engineer @ Parity_

<!-- .element: style="margin-left: 20px;" -->

- I am a Subject matter in Academy Support
- <- This is my ETHDenver PFP
- I **_love_** to hike!

_[Twitter](https://twitter.com/NukeManDan) // [LinkedIn](https://linkedin.com/in/danwshields) // [Email](mailto:dan.shields@parity.io)_

</div>
</div>

---

### Outline

<!--
You can reference slides within this presentation like [this other slide](#at-the-end-of-this-lecture-you-will-be-able-to) by use of the header title.

Please make your lecture precise.

- Limit the main points in a lecture to five or fewer.
- Create effective visuals, analogies, demonstrations, and examples to reinforce the main points.
  {TAs and the Parity design team can assist! Please let us know marking an item here as `TODO`}
- Emphasize your objectives and key points in the beginning, as you get to them, and as a summary at the end.

-->

- [Pre-requisites](#pre-requisites)
- [The "Why" of this lecture](#the-why-of-this-lecture)
- [Exercise: Explore Jupyter Lab](#exercise-explore-jupyter-lab)
- [Workshop: Open an Academy PR](#workshop-open-an-academy-pr)
- [Best Practices for SMEs Content](#best-practices-for-smes-content)
- [Conclusion](#conclusion)
- [Next Steps](#next-steps)
- [References](#references)

---

## Pre-requisites

_An Open Mind_

---

### At the end of this lecture, you will be able to:

- Navigate the Academy github repo
- Use the tools available to build your lectures and workshops
- Have high confidence in your ability craft your academy materials

---

## The _Why_ of this lecture

_Problem_: Many are uncomfortable getting started with content generation.

_Solution_: Meta-lecture to demonstrate **_SOME_** tools at your disposal!

<hr>

We need to craft on **_practical, hands-on learning_** wherever practical.
We all strive to **_demonstrate application_** of conceptual lecture content.

Notes:
These tools are SUGGESTIONS!
We trust you to use the tools and techniques you feel best in, and if these are not them we can work with you to get the tools you need in place!
Class take mind: I am giving you a lecture with all the tools that I like and can help you use!

---

### A paraphrased note from Gav:

<div class="centered" style="font-size: 0.8em"> <!-- Gotcha: You Need an empty line to render MD inside <div> -->

> One of my most memorable and useful lecture series in my undergrad degree was the very first course I took: principles of programming.
> The lecturer had their laptop connected to a screen and the course was basically teaching all the various programming concepts via the Scheme language (simple and highly interactive which also worked naturally through a REPL shell). <br><br>
> Despite having been coding for 10 years before, I found it a highly engaging and enlightening lecture series, primarily due to this extremely immediate and concretely demonstrative nature of the content.
> Rather than having a just slide show going on in the background like every other lecture series, he actually showed us what he meant as he was explaining it.<br><br>
> ...Of course some lectures since they don't trivially translate to code, but for most it should be doable!

</div>

Notes:
"the lecture must make it look effortless so they can demonstrate as they teach. if they can't then they haven't gotten it right."
This is what I have been toiling to make easy for you to do, using the best-class tooling loved by many CS students.

---

## Exercise: Explore Jupyter Lab

Notes:
OPEN the jupyter-lab and jump to the [paired notebook for this lecture](../jupyter-notebooks/how-to-use-academy-resources.ipynb)

---v

<div class="flex-container">
<div class="left"> <!-- Gotcha: You Need an empty line to render MD inside <div> -->

### Install EvCxR Jupyter Lab

- Install Python {if needed} ([instructions here](https://wiki.python.org/moin/BeginnersGuide/Download))
- Install [`evcxr_jupyter`](https://github.com/google/evcxr/tree/HEAD/evcxr_jupyter#installation)
- Open the [evcxr_jupyter_pba_example.ipynb](/content-templates/evcxr_jupyter_pba_example.ipynb) with the tool of your choice:

</div>

<!-- Put no content here -->

<div class="right">
<!-- Gotcha: You Need an empty line to render MD inside <div> -->

```sh
# Launch Jupyter Lab
jupyter-lab

# Navigate to the source notebooks in
# `content-templates/jupyter-notebooks` of the Academy repo
```

</div>
</div>

Notes:
You might need to read the CONTRIBUTING.md guide to get links to install node and yarn themselves.
Other tooling install and basic use is described there too.
You MAY use ExCvR in a shell only, not a notebook. Notebooks are by far better in every way.

---

## Workshop: Open an Academy PR

### _Time to get to work!_

Notes:
Grab your machine!

---v

<div class="flex-container">
<div class="left"> <!-- Gotcha: You Need an empty line to render MD inside <div> -->

### Install & launch slides

- Open https://github.com/paritytech/polkadot-blockchain-academy
- Clone the repo
- Open up the [CONTRIBUTING.md](../../CONTRIBUTING.md) doc
- Follow instructions to setup `node` and `yarn` as needed
- Open up the HTML slides in a browser

</div>

<!-- Put no content here -->

<div class="right">
<!-- Gotcha: You Need an empty line to render MD inside <div> -->

```sh
# Get the right node installed
nvm i

# Get the dependencies
yarn

# Start the show!
yarn start

# Default has http://localhost:1948/ as the hosted slides version
# Open this in a browser!
```

</div>
</div>

Notes:
You might need to read the CONTRIBUTING.md guide to get links to install node and yarn themselves.
Other tooling install and basic use is described there too.

---v

<div class="flex-container">
<div class="left"> <!-- Gotcha: You Need an empty line to render MD inside <div> -->

### Open a PR for Academy content

- TODO: add some instructions here!

</div>

<!-- Put no content here -->

<div class="right">
<!-- Gotcha: You Need an empty line to render MD inside <div> -->

```sh
# TODO: Unfinished as of yet!
```

</div>
</div>

Notes:
Use the IDE you like, or not. I will demo VSCode.

---

## Best Practices for SMEs Content

---v

## Document extensively

- Don't worry about being too verbose!
  - The more source we have and insights into the _what & why_ of the content helps us!
- Please use `Notes` in slides and comments all over
  - For you, and for those that maintain the content and teach it next time

Notes:
TAs are here to help ask questions to help us curate content.

- You will be here for round one, but we will run 3 to 4 times A YEAR!
- Please help those that take over and may not be SMEs at your level can deliver this content.

---v

## Next steps

### _In your content slides/notebooks!_

Compile an **annotated** list of:

- Topics students should consider learning about independently
- Examples of the concepts covered & applied to a project / case studies
- Useful resources related to the lesson
- Reference other slides/materials by relative directory in this repo,
  - Like the [copy-paste slide templates](../../content-templates/slides/copy-paste-reveal-template-slides.md)

Notes:
there is no such thing as too much! anything with a NOTE ABOUT WHY IT'S USED!

ANNOTATE!

---v

## References

### _In your content slides/notebooks!_

Compile an **annotated** list of URLs to source material referenced in making these lessons.
Ideally this is exhaustive, it can be cleaned up before delivery to students, but must include _why_ a reference is used. For example:

- [ss58-registry](https://github.com/paritytech/ss58-registry) - A list of known SS58 account types as an enum, typically used by the Polkadot, Kusama or Substrate ecosystems.
- [wiki on parathreads](https://wiki.polkadot.network/docs/learn-parathreads) - A description of the parathread model.

---v

### **PLEASE cite your references!**

For now, make a note however you like indicating sources _in context_
<br><br>

- [Zotero](https://www.zotero.org/) integrations {latter on}
  - We will have a linked bibliographic source list in all class materials
  - Here is the [Library of sources so far](https://www.zotero.org/groups/4677582/polkadot-blockchain-academy/library)

Notes:
Dan is using Zotero, and will help make things polished, up to accedemic standars of biligraphic references/citations. just note the sources used and where to cite them!

---

## Teaching Assistants are here to support!

### _We are all here for **anything** you need_

Notes:
Please lean into us, we are deeply passionate about helping you shine as an instructor!
My mission is to make help you be a fantastic instructor with minimal effort.

---

<img style="height: 980px;" src="../../assets/img/0-Shared/placeholder/carry-you.png"/>

Notes:
TAs are not SMEs!
We cannot write the content as well as you can, and in the end you ar every very likely to deliver this first run of the Academy.

---

<img style="height: 600px;" src="../../assets/img/0-Shared/placeholder/we-can-do-it.png"/>

_Team work makes the dream work!_

Notes:
We have a month! Let's get moving!
We are gonna make a HUGE impact in Web3 with this content, for us and for the ecosystem.

---

## Conclusion

- We demoed how you might give a lecture in three ways:<br>
  Pure slides // Jupyter notebooks // VSCode
- TAs are _here to help you succeed_!
- We have _one month_ to get this done... **time to start grooving**!

Notes:
In this _meta-lecture_ we experienced a lot of tools and techniques to make your lectures and wow students!

---

## Next steps

- Open a PR with any minor change to your content, and request a review from the TA that is assigned to help you
- Get support from TAs
  - Requests: Open an issue on Github {more tools, clarifications, planning}
  - Ask: Connect on Element to TAs _at any time_! In shared Academy channels on a DM.
  - Schedule: setup calls with your TA to ask questions directly and work on content!

---

## References

- [These slides](TODO) - in your repo (once that PR is approved and merged ;) )
