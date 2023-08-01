---
title: Practical Rust Project v2 content changes
updated: 2022-09-18 12:57:59Z
created: 2022-04-05 22:12:26Z
---

# General
- [ ] Cargo add is now a built-in command
- [ ] Test all code again with the latest Rust version
- [ ] Make sure all shell example start with `$`
# 1. Welcome to the World of Rust
- [ ] What's changed in the 2nd edition.
- [ ] Add a note on Rust 2021 and version mentioned in [Practical Rust Projects, 2nd edition#update-code-example](../00.TEMP_Rust%20project/Practical%20Rust%20Projects,%202nd%20edition.md#update-code-example)
- [ ] Update on the alternative crates
- [ ] Explain the long output code block line wrapping symbol [LaTex Listing#wrap-long-lines-and-have-an-arrow-when-it-wraps](../undefined#wrap-long-lines-and-have-an-arrow-when-it-wraps)
- [ ] Update the Rustc version
# 2. Building a Command-Line Program
- [ ] Mention clippy and IDE tool? Maybe spread to mulitple chapters. Or a aside box at each chapter.
- [x] Update on the alternative crates
# 3. Creating Graphical  User Interfaces (GUIs)
- [ ] Upgrade to GTK 4? => No, stay with GTK3 because
	1. GTK4 library is not available on Ubuntu 20.04
	2. GTK4 doesn't have Glade
	3. GTK4 is not yet widely adopted by, for example, Ubuntu ([Ubuntu 22.04 LTS Aiming For GNOME 42, Avoiding GTK4 Where Possible - Phoronix](../undefined))
	4. GTK3 is still actively maintained [GTK 4.0 – GTK Development Blog](../undefined)
- [ ] Update on the alternative crates
	- [ ] GTK4
- [ ] Mention `gtk::Application` builder pattern?
- [ ] [ Rust GUI libraries](../undefined)
- [x] [Flutter-rs](../undefined) is dead
- [x] IUP rust is not updated for a long time
# 4. High-performance Web Frontend using WebAssembly << from the web book
- [ ] `wasm-pack test` webdriver tests?
- [ ] Update on the alternative crates
- [ ] Use `trunk` tool for `yew` => [Yew](../undefined)
- [ ] Use the REST API frontend as yew example, do not rewrite the image processing example in yew
# 5. Building REST APIs << from the web book
- [ ] Update on the alternative crates
- [ ] Hello world: use the new `.service()`+`get` macro
- [ ] Handlebars now requires a `dir source` feature to use register_termpaltes_directory
# 6. Going Serverless << from the web book
- [ ] Using the new AWS SDK for Rust and Rust runtime for AWS Lambda
- [ ] Update on the alternative crates
# 7. Building a Game
- [ ] Use a different game engine because the one used in 1st edition seized development => Most likely [Bevy](../undefined) becauser of the similarity to [Amethyst - Rust game engine](../undefined) and popularity
- [ ] Mention [Rust GameDev WG](../undefined)
- [ ] Mention new projects in [Game Engines | Are we game yet?](../undefined) 
- [ ] Update on the alternative crates
- [ ] [Build Rust game with Bevy ](../undefined)
# 8. Physical Computing in Rust
* Maybe add a section on using WebAssembly on RPi
- [ ] Update on the alternative crates
- [ ] rust_gpio upgraded to 0.2.1
- [ ] Use RPi 4
- [ ] Some mut/non-mut changes
# 9. Artificial Intelligence and Machine Learning 
* Maybe add a section on deep learning
- [ ] Update on the alternative crates
- [ ] gnuplot `fg.show()` now returns an result, add `?`
# 10. What else can you do with Rust?
- [ ]  Remove the web part that is inlcuded in 2nd edition
- [ ] Crypto use case?
- [ ] Update on the alternative crates
