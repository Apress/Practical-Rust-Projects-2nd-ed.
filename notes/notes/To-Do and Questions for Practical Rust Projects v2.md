---
title: To-Do and Questions for Practical Rust Projects v2
updated: 2022-04-27 09:47:19Z
created: 2022-03-31 21:04:30Z
---

- [ ] Run Rustfmt on the example code repo
- [ ] REST API
	- [ ] Check the status of `::data` to `::app_data` migration
	- [ ] `diesel::result::Error` has no `Serialize` => See the end of https://actix.rs/docs/databases/ ?
- [ ] Replace any path `/home/shinglyu/workspace/` in output by `~/`
- [ ] Re-evaluate the alternative libraries for each chapter
- [ ] Research on [Style Guide](../00.TEMP_Rust%20project/Style%20Guide.md)
- [ ] Shrink the book margin for accurate code width
- [x] Commit example code to github as private repo
- [ ] cargo-add => cargo-edit
- [ ] Add citation?
- [ ] Add README to code
- [ ] Code release checklist
- [ ] Chapter release checklist
- [ ] Run example code through clippy?
- [x] Estimated timeline?
	- [x] How long did it took me for a chapter in v1?
- [x] Should I continue to use LaTex? => Yes
- [x] What needs refresh?
- [ ] CLI
- [ ] GUI
	- [ ] GTK4 support? ([gtk-rs](../undefined))
	- [ ] Modern GUI?
	- [ ] Alternatives: [iced-rs - A cross-platform GUI library for Rust, inspired by Elm](../undefined), [Tauri](../undefined), egui, [linebender/druid: A data-first Rust-native UI design toolkit.](../undefined)
- [x] Game
	- [x] Amethyst update? => [Amethyst is dead](../undefined#dead-httpsamethystrspostsamethyst-starting-fresh), maybe check [Bevy](../undefined)?
- [ ] Embedded
	- [ ] What can I demo on baremetal RPi?
	- [ ] Maybe rebrand it to IoT and use WASM as in [Programming WebAssembly with Rust: Unified Development for Web, Mobile, and Embedded Applications by Kevin Hoffman](../undefined)?
- [ ] AI/ML => [Awesome Rust Machine Learning](../undefined), [Are we learning yet - Rust](../undefined), [e-tornike/best-of-ml-rust: 🏆 A ranked list of awesome machine learning Rust libraries.](../undefined)
	- [ ] New classical ML library?
	- [ ] Deep learning model?
- [x] REST API
	- [x] Web Framework change? Is Actix still popular? => Yes, keep using Actix, see [A comparison of some web frameworks and libs written in Rust#observations](../undefined#observations)
	- [ ] Maybe include some WebSocket content to it. Like chat, or push notification
- [x] WebAssembly frontend
	- [x] Still use Yew? => Yes, see [A comparison of some web frameworks and libs written in Rust#observations](../undefined#observations)
- [x] Serverless 
	- [x] AWS SDK? => [AWS SDK for Rust - Announcement blog post](../undefined)
	- [x] New AWS Rust Runtime?A => [AWS Rust Lambda Runtime](../undefined)
- [x] Rust 2021 changes? => [Rust 2021 - The Edition Guide#migration](../undefined#migration)
- [ ] Async IO merge into certain chapter? => See if [Actix-web](../undefined) is using async?
- [ ] New ideas? 
	- [ ] Linux kernel driver?
	- [ ] Carlo Milanesi's book?
- [ ] Mention clippy and IDE tools? => A column for tools?
- [ ] Remove all emails in Cargo.toml example code

# Question for editor
- [ ] 2nd person voice?
- [ ] Code block without caption
- [ ] Code visible space => good for layout?
- [ ] Rename Chapter 1 to Preface (without number)
- [ ] Rename the title