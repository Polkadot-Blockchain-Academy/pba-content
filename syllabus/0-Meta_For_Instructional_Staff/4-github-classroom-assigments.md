## GitHub Classroom How-to

[GitHub Education](https://education.github.com/) provides a lot of nice features for formal educational institutions and their students.
One of the tools we can leverage in the PBA is [GitHub Classroom](https://classroom.github.com/) that allows for **using a base git repo as a source for assignments**.
Our [Rust Entrance Exam](https://github.com/Polkadot-Blockchain-Academy/Rust-Entrance-Exam) uses this feature, so we can have individuals or groups work privately from each other, while staff can see and assess everyone's work. <!-- markdown-link-check-disable-line -->
It also can allow for automated testing (via a version of GitHub Actions / CI).

> If you want to create an assignment, please let the **owners of the [PBA org](https://github.com/orgs/Polkadot-Blockchain-Academy/people?query=role%3Aowner)** know so, as you will need to get permissions updated.
> Best contact at the time of writing is Nuke: https://matrix.to/#/@nuke:parity.io

## Create a activity or assignment

The basic workflow for this for an instructor is to:

1. Create a **private** stand-alone repository in the [PBA org](https://github.com/orgs/Polkadot-Blockchain-Academy/ for an assignment or activity.

   1. Ensure a README exists with clear instructions.
   1. Create a _dedicated branch_ to write up a solution.
   1. Integrate unit and/or integration tests that students can run to check their work.<br>
      > Note: these can be run as [GitHub Actions](https://docs.github.com/en/actions) on student's pushing commit's to their work, like a `cargo test ...` or script for example. <!-- markdown-link-check-disable-line -->
      > See [docs on autograding](https://docs.github.com/en/education/manage-coursework-with-github-classroom/teach-with-github-classroom/use-autograding) <!-- markdown-link-check-disable-line -->
   1. Ensure `main` of the repository is the assignment as it should be delivered to students.
   1. In the github repo's general settings, switch the repository to a [template](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-template-repository). <!-- markdown-link-check-disable-line -->

1. Create a new assignment on [GitHub Classroom](https://classroom.github.com/) under the right one, there is one per cohort.
   1. Unless otherwise needed, best to keep default settings as the student repo will be private and not give them admin to it.
      Don't add a "supported editor".
   1. Select your template repository for this assignment.
   1. Enable a Feedback PR
   1. Enable the invite link, and give it a test yourself!

## Sending, working on, and finishing assignments

With the new invitation link, anyone can generate a fork of the repo with zero history (fresh git history).
Their repository will default to private to let them work on it without other student's able to view their work.
Each push to `main` on their copy of the assignment will, if configured, trigger CI (autograding) to be run, along with any workflows that exist already in the assignment.

All student repo's should have a `PR #1` opened when they accept the classroom assignment as a feedback from instructor -> student of their work based on the generated 0th commit on the assignment & the head of `main`.
Feel free to use this as you like, and remind everyone to `@` you for attention if they need it there, as its otherwise likely to be missed.

If configured, at the deadline students can still push to their fork, but there is a collection of a grade from the CI.
The classroom assignment page _does_ show the `Submitted` or `Not Submitted` and if they had work in, links to the last commit before the deadline.
Changing the deadline also re-evaluates, but it's not clear if that is at the right commit or the highest one.
It will be up to graders to decide on deadline compliance by either removing write access to each student's repo, and/or to inspect and grade the highest commit before the deadline.

### Manual grading

As the CI autograding is likely not sufficient, grading each repo is required.
In order to download all assignments it's likely we want to use https://classroom.github.com/assistant .
It's an electron app that lets to select an assignment and select some or all repos to clone in a batch into a single dir locally, with repos being only the github username of the student.

> Note it's _old and not maintained_ but the [2.0.3 .deb release](https://github.com/education/classroom-assistant/releases/tag/v2.0.3) was working on 20.04 ubuntu (22.04 [likely broken](https://github.com/education/classroom-assistant/issues/235))

## Need more help?

The PBA team is here for you!
Please reach out to ask us anything!

### GitHub's Docs on Classroom

- [Basics of setting up GitHub Classroom](https://docs.github.com/en/education/manage-coursework-with-github-classroom/get-started-with-github-classroom/basics-of-setting-up-github-classroom) <!-- markdown-link-check-disable-line -->
- [Create an individual assignment](https://docs.github.com/en/education/manage-coursework-with-github-classroom/teach-with-github-classroom/create-an-individual-assignment) <!-- markdown-link-check-disable-line -->
- [Create a group assignment](https://docs.github.com/en/education/manage-coursework-with-github-classroom/teach-with-github-classroom/create-a-group-assignment) <!-- markdown-link-check-disable-line -->
