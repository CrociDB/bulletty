---
title: Install
summary: Install bulletty
show_datetime: false
---

# Install

## 🟩 Stable Version

### Prebuilt binary

[Download the latest version](https://github.com/CrociDB/bulletty/releases/latest) of **bulletty** through GitHub.

**bulletty** runs natively on all the major three platforms: **Linux**, **MacOS** and **Windows**. One thing to be aware of though is that it does make use of some symbols found in [NerdFonts](https://www.nerdfonts.com/), so it's highly recommended to have it setup in your terminal emulator.

### Through _Cargo_

Considering you have `cargo 1.90+` installed in your system:

```shell
cargo install bulletty
```
[bulletty on crates.io](https://crates.io/crates/bulletty)

## 🌃 Nightly Builds

[Download a nightly build](https://github.com/CrociDB/bulletty/releases)

A nightly build can be more unstable, but it's very appreciated if you want to test new features.

Another option is installing a nightly version from `cargo`:

```shell
cargo install --git https://github.com/CrociDB/bulletty.git
```

## 👩‍💻 Getting the source and building it

```shell
git clone https://github.com/CrociDB/bulletty.git
cd bulletty
cargo build --release
```
