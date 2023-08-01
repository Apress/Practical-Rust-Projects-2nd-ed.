---
title: Style Guide
updated: 2022-06-04 19:23:32Z
created: 2022-04-01 21:14:59Z
---

# Questions
- [ ] How to capitalize chapter title?
- [x] When mentioning function names, do we use `myFunc()` or `myFunc`? => Use `myFunc()`
- [ ] Do we add a `# filename` in every code listing? => Depends if the filename is important or not clear from the context
- [ ] `Cargo.toml` author: name only, no email
- [ ] Highlighting part in the code, use `// [1]`?
- [x] When to use code listing, when to use inline verbatim code? => Some book use Listing for every piece of code. Some book use no Listings, only indent the code block and use smaller fonts. Some book has a mix: indented and smaller font but no Listing for commandline input/output. Listing for long piece of source code (e.g. Mastering Bitcoin) => I decide to NOT have Listings caption in most cases, unless if it's a very long piece of code that does not fit into the flow of text. 
- [ ] File name use code font?
- [ ] How to reference Listing?
- [ ] Figure caption use normal captalization?
- [ ] Code that is not relavent is hidden with `// ...`? 
- [ ] How to wrap lines for wide console output?
- [ ] Terminology: STDIN/stdin/Stdin?
- [ ] Code example clippy setting?
- [ ] Links, use footnote or citation?
- [ ] 2nd person? 1st person?  => 2nd person

# Style Guide
* Use code font for commandline tool name
* Use code font for Rust crate name
* When referencing a function, include the `()`. Unless the parameter needs to be highlighted, ignore the paramters.
	* Good: `MyFunction()`
	* Bad: `MyFunction`, `MyFunction(not_important_arg)`
* When referencing a member function of a trait (e.g. `Car.drive()`), keep the `.` (e.g. "The `.drive()` function allows you to drive the `Car`")
* Add the filename to the code block using a comment at the first line:
    ```
	// src/main.rs
	fn main() {...}
	```
* Use active voice when talking about a piece of code doing some operations:
	* Good: The `StructOpt` crate generates a command-line parser
	* Bad: The `StructOpt` give you the ability to generate a command-line parser
* If the code contains any email or PII, remove it if it doesn't break the code
* For fake identity, use:
	* Code path: `/home/user`
	* Email: `user@domain.com`
* For terminal command, add the prompt `$`:
	```
	$ ls
	```
* Do NOT include a period at the end of a image or listing caption.
# Spellings
* command-line (bad: commandline)
* boolean (bad: Boolean) see [The Universe of Discourse : "Boolean" or "boolean"?](../undefined)
* 

# Software tools
* Use `apt` instead of `apt-get` ([Difference Between apt and apt-get Explained - It's FOSS](../undefined).

# Rust
* Use `Option<T>` when refering to an instance of `std::option::Option`. e.g. "Wrap the `u64` with `Option<T>` so it becomes optional"

# My LaTex template:
* Use the `language=output` for listings that are terminal outputs. The long lines will be wrapped with arrows

# References:
* A compilation of different style guides: https://draft.dev/learn/technical-writer-style-guides
* A List Apart guide: https://alistapart.com/about/style-guide/
* 