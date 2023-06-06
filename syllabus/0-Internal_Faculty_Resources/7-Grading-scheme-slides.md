---
title: New Academy Grading Scheme
description: Research and proposal from Nuke
duration: 30 minuets
# PBA has a theme: "reveal-md/PBA-theme.css", alternatively, you can use a named default like "night" from this list: https://github.com/hakimel/reveal.js/tree/master/css/theme/source
# Add custom css files for your slides here, comma separated:
separator: "\r?\n---\r?\n"
verticalSeparator: "\r?\n---v\r?\n"
# Below can be any of these: https://revealjs.com/config/
revealOptions:
    transition: "concave" # animation between slides = none/fade/slide/convex/concave/zoom
	backgroundTransition: "fade" # background swap between slides = none/fade/slide/convex/concave/zoom
	slideNumber: true
	controls: true
	progress: true
---

<!-- .slide: data-background-color="#4A2439" -->

# New Academy Grading Scheme

---

## Goals (1) 🎯

- **Objective, easy to concretely define achievement, and concrete measures on assessment - qualitative and quantitative.**
- Grading to be uniform and transparent for _everyone_.
  - No normalization. especially to fit a target pass/fail certification rate.
  - Rubrics and complete solutions (A "Perfect" one, and ideally various levels of score categories as example for each) for all graded material.
  - A full test suite for code to check functionality
  - **map exactly with learning goals & outcomes**
  - "fuzzy" topics like code quality & "beauty" need to be confined outside of hard skills & competencies in any assessments/scores/pass or fail.

---

## Goals (2) 🎯

- Focus on growth & learning, not a number for grades.
  - [Learning, not earning! 📺](https://www.youtube.com/watch?v=CnSkOXe90WI)
  - **Foster intrinsic motivation & drive.**
  - No points system can be high "rigor", focus must be on quality of work.
  - Encourages unique work & creative thinking
  - Better granularity via skills and competencies, vs. "some number" that encompasses overall growth/accomplishment.
- Feedback direct from SME to all students on all graded assignments (at least option on request, this can be TA's primary role)

---

## Goals (3) 🎯

- Beta testing to be complete before students are tasked with any materials. Especially those graded.
- Flexibility to break "out of the mold" to follow passions & dig deep: thesis track.
- Enable a pathway to thesis based model for multi-track future with tools and techniques used.
  - All certs granted at the closing ceremony would only be for participation! Only after the fact can anyone get a specific distinctions (engineering, founder, etc.) after a thesis defense.
  - The exception _may_ be the main engineering certification - for Application Engineers (parachain and solochain engineering)

---

## Anti-goals 🙅

- "Shame" under-achievement that leads to exiting the ecosystem. Maximize continued high-impact involvement post academy for _all participants_.
- Fail fo the sake of specific % not certifying
- "Gossip" and personal gripes/bias about students leading to bias & keep (by default, without good cause) _any_ subjective details on students out of the picture _between graders_. (Thesis committee would break this rule, running after in-person cohort is complete)

- unclear and subjective points systems with no clear definition that leads to grader interpretation of what a score means.

---

## PR Model for Grading (1) 🧐

- A student README that discusses the work that SME will review
  - calls out things the student wants the SME to review in work directly
  - reflection on the work: key learning, things still to do, unresolved questions/issues the student had
- Unit tests / CI to run and flag pass/fail (like qualifier)
- Classrooms feedback PR can be used to comment directly on the work of each student easily.
  - Issue template is the grading rubric with tasks complete or incomplete to define if competency is met....? (next slide)

---

## PR Model for Grading (2) 🧐

- Competency checklist -> score (1 to 4 overall?).
  - 2 is passing all minimal requirements. 3 for bonus IDed ones. 4 for grader discretion of going above the call of the assignment (given the level expected at this point)
  - build competencies as pass/fail -> minimal version, inter,. adv. so doing adv. gets all 3.
  - comp. are for the course, not the assignment - there can be overlap! build over course, not locked per assignment.
  - ~~Consider making public? cannot as solutions are private to PBA & student.~~

---

## [Developing Quality Assessments 📺](https://www.youtube.com/watch?v=CnSkOXe90WI)

- Blooms taxonomy
- WHY give this assignment? in context with content.
- assess on:
  - process (thinking through) and/or product (shipped solution {code})
  - express ideas concisely and coherently
  - convergent (coming to conclusion based on given) or divergent (hypothesis from predictions & unstated things)

---

## [Best Practices for Grading Objectively and Efficiently 📺](https://www.youtube.com/watch?v=hiUXBr4sgnM)

---

## [Alternative Grading Frameworks 📰](https://teaching.berkeley.edu/resources/course-design-guide/design-effective-assessments/alternative-grading-frameworks)

Also [this one 📰](https://academictechnologies.it.miami.edu/explore-technologies/technology-summaries/alternative-grading/index.html) among many other less known frameworks.

---

## [Grading for Equity 📰](https://www.insidehighered.com/views/2020/01/27/advice-how-make-grading-more-equitable-opinion)

- Subjective criteria to minimum or zero
- Transparent scoring -> Are mathematically accurate to validly describe a student’s level of mastery. They apply a more proportionately structured 0-4 scale instead of the 0-100 scale, which is mathematically oriented toward failure. They also use sound mathematical principles that reflect recent performance and growth instead of averaging performance over time.
- No normalization
- Support hope and a growth mind-set. They allow test/project retakes to emphasize and reward learning rather than penalize it, and they override previous scores with current scores that build learning persistence.
- “Lift the veil” on how to succeed. They create explicit descriptions of what constitutes demonstration of content mastery through rubrics or proficiency scales. In addition, they simplify grade books and expand the methods of assessments to generate more accurate feedback and reporting about each student’s learning relative to the expected outcomes.

---

## [Specs Grading 📰](https://www.insidehighered.com/views/2016/01/19/new-ways-grade-more-effectively-essay)

- Pass fail only (like PRs!)
- Bundles to achieve levels

---

## [Ungrading 📰](https://www.insidehighered.com/advice/2017/11/14/significant-learning-benefits-getting-rid-grades-essay)

- Reflection and Dialogue: Ungrading builds upon similar aspects of specifications and contract grading. Assignments provide clear instructions, although not necessarily criteria or contracts, for students to follow. Instructors should also offer students flexibility with assignment deadlines and provide opportunities for revision. Ungrading does encourage instructors to have more open conversations with students about their performance, whether it is through bi-weekly conferences, feedback surveys, or asking students outright what grade to put in the system at the end of the term (Blum & Kohn, 2020). These conversations in addition to other self-reflective exercises (i.e. minute tickets, process letters, peer feedback, etc.) require students to think critically about what they’ve learned and articulate how they have developed their knowledge and skills throughout the semester.

---

## Contract Grading

TODO

---

## [Competency Grading 📺](https://www.youtube.com/watch?v=YQInjf8UjOo)

Great explainer in [three](https://www.nciea.org/blog/what-do-i-need-to-know-about-competency-based-grading) . [part](https://www.nciea.org/blog/what-do-i-need-to-know-about-competency-based-grading-2) . [serries](https://www.nciea.org/blog/what-do-i-need-to-know-about-competency-based-grading-3)

---

## Thesis Driven Model

TODO
