# Grading

need 3 inputs:

- template (base code and solutions)
- grading / testing WASM src
- student code

1. gh classrooms CLI to grab all repos into named folders
1. scripts (sh and sh) operate to these
   1. pre is just for sending tests updated for students
   1.
   1.
1. runtime has solutions baked in (kiz branch)
1. integration tests run against the STUDENT
   1. could be pulled into it's own bin (black box for testing against)
1. grading final py hard coded for what you want to generate (calc grades, csv grades, push grades)

## TODO

- [ ] migrate from slides repo to own repo.
- [ ] clean up and comment scripts.
- [ ] (nice, low priority) refactor the grading scripts that running tests into a proc-macro... great if e2e rust.
