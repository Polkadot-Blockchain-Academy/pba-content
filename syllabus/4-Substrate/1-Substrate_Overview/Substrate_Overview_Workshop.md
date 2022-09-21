# Cyclic dependencies

In the Substrate codebase you'll notice that everything is organized into independent separate crates to avoid running into cyclic dependency issues.
Primitive types and traits have been refactored out into their own separate crates, as well as other client and runtime components.

This workshop will help you understand the way Substrate's codebase is organized and what running into a cyclic dependency error would look like.

## Steps

1. Create a new Rust project.
1. Create a crate A with a set of traits that use traits from another crate B.
1. Create a new trait in crate B that uses traits from crate A.
1. If you were successful, the compiler will give you a `error: cyclic package dependency:` error.
1. Refactor your code into separate crates to fix it.
