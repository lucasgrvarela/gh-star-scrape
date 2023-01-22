# What is this project about?

This is just a simple test performing some web scraping on Github Stars page of my profile using Rust.

The idea is to get the project link and open in Google Chrome for a GH Star Page - containing almost 30 projects per page.

I used this small tool to open the links for me and I did manual review if the project was still maintained or not, if not I did unstar it.

# Usage

You simple pass as argument your Github star profile page. The code is not handling multipage.

```rust
cargo run "https://github.com/lucasgrvarela?tab=stars" # page 1

cargo run "https://github.com/lucasgrvarela?after=Y3Vyc29yOnYyOpK5MjAyMi0xMi0xNlQxODo0Nzo1My0wMzowMM4WWnHk&tab=stars" # page 2
```