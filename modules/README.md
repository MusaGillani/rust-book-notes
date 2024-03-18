# how many crates are there in a project?

There is default crate in every project

> if there is a main.rs file in the src dir
> rust will create a binary crate with the name as the project
> main.rs will be the root file for that crate

> if there is a lib.rs file in the src dir
> rust will create a library crate with lib.rs as the root

> to add more bin crates to this project, create bin dir in src any file in this bin dir is considered a bin crate
