---
title: Practical Rust Projects, 2nd edition
updated: 2022-10-01 18:23:35Z
created: 2022-03-03 19:56:36Z
---

[toc]

---
# [Letter to stop](../undefined)
# [Google Drive](https://drive.google.com/drive/folders/1LCiMixbhdS36_1cvkdpxFuTrP2j80BNY)
# [To-Do for Practical Rust Projects v2](../00.TEMP_Rust%20project/To-Do%20and%20Questions%20for%20Practical%20Rust%20Projects%20v2.md)

# Open Questions => Moved to [To-Do and Questions for Practical Rust Projects v2](../00.TEMP_Rust%20project/To-Do%20and%20Questions%20for%20Practical%20Rust%20Projects%20v2.md)

# [Book Chapter Checklist](../undefined)
# Estimated timeline
* Need complete rewrite: 
	* Game: 6 weeks
	* Serverless: 4 weeks
* I want to expand if I have enough time:
	* AI/ML: 4 weeks (add deep learning?)
	* Physical computing: 4 weeks (add WebAssembly on RPi or bare metal programming?)
* Only update library version:
	* CLI: 3 weeks
	* GUI: 3 weeks (or 4 weeks if we add a section for IMGUI vs Retained mode)
	* REST API: 3 weeks
	* Web Frontend: 3 weeks

## Schedule
* 2022/04/17 CLI
* 2022/05/22 Game
* 2022/06/19 Serverless
* 2022/07/03 REST API
* Buffer time
* 2022/07/31 **First 3 chapters**
* 2022/08/28 Physical computing
* 2022/09/25 AI/ML
* 2022/10/16 GUI
* 2022/11/06 Web Frontend
* 2022/11/27 Last chapter conclusion
* 2022/11/27 First chapter preface
* Buffer time
* 2023/02/28 **Final deadline**

# Proposed TOC
Moved to  [Practical Rust Project v2 content changes](../00.TEMP_Rust%20project/Practical%20Rust%20Project%20v2%20content%20changes.md)
# [Practical Rust Project v2 content changes](../00.TEMP_Rust%20project/Practical%20Rust%20Project%20v2%20content%20changes.md)


# Update code example
## Using Rust 1.61, version 2021
* 1.61 is requred by `cursive`
```bash
# Get the latest Rust version
rustup update
cd ~/workspace/practical_rust_v2
# Get the example from old book repo
git checkout backup-old <chapter_name>
git commit -m "feat: old chatper X"
cargo fix --edition
vim Cargo.toml # Update the edition field to "2021"
# Remove the email in Cargo.toml author 
cargo build && cargo test # Check if everything is OK
git commit -m "feat(<chapter>): upgrade to Rust 2021"
cargo upgrade # Requires cargo-edit
cargo build && cargo test # Check if everything is OK
git add Cargo.toml
git commit -m "feat(<chapter>): upgrade dependencies"
```
See [cargo-edit: A utility for managing cargo dependencies from the command line](../undefined)

# [Style Guide](../00.TEMP_Rust%20project/Style%20Guide.md)

# Files
## Source
* Book LaTex: `~/workspace/rust_applications_book/drafts/book1-v2`
* Example Rust code: `~/workspace/practical_rust_v2`
## Rendered
* Drafts PDF for sending to iPad for proofread: `/home/shinglyu/Dropbox/0.work/practical_rust_book_v2_proofread`

---
# See also: 
* [Practical Rust Projects source and Practical Rust Web Projects source code](../undefined) 
* [Practical Rust Projects, 1st edition](../undefined)
* [Practical Rust Web Projects, 1st edition](../undefined)

# Contact: 
* Project coordinator: Shonmirin P.A. <shonmirin.pa@springernature.com>
* 	Gryffin Winkler <gryffin.winkler@springernature.com>
Keywords; book, apress