---
title: Install
summary: Install bulletty
show_datetime: false
---

# Install

<a href="https://repology.org/project/bulletty/versions">
    <img src="https://repology.org/badge/vertical-allrepos/bulletty.svg" alt="Packaging status" align="right">
</a>

## 🟩 Stable Version

### Prebuilt binary

[Download the latest version](https://github.com/CrociDB/bulletty/releases/latest) of **bulletty** through GitHub.

**bulletty** runs natively on all the major three platforms: **Linux**, **MacOS** and **Windows**. One thing to be aware of though is that it does make use of some symbols found in [NerdFonts](https://www.nerdfonts.com/), so it's highly recommended to have it setup in your terminal emulator.

---

## 🐧 Linux

### Arch Linux

**bulletty** is available in the Arch Linux `extra` repository. This also applies to Arch-based distributions such as **Manjaro** and **Parabola**.

```shell
pacman -S bulletty
```

[View package on Arch Linux](https://archlinux.org/packages/extra/x86_64/bulletty/)

### Nix / NixOS

**bulletty** is available in **nixpkgs**. You can install it using the flakes-style command:

```shell
nix profile install nixpkgs#bulletty
```

Or using the traditional command:

```shell
nix-env -iA nixpkgs.bulletty
```

[View package on nixpkgs](https://github.com/NixOS/nixpkgs/blob/nixos-unstable/pkgs/by-name/bu/bulletty/package.nix)

### Snap

[![bulletty](https://snapcraft.io/bulletty/badge.svg)](https://snapcraft.io/bulletty)

**bulletty** is available as a Snap package, which works on Ubuntu, Debian, Fedora, openSUSE, and many other Linux distributions that support [snapd](https://snapcraft.io/docs/installing-snapd).

```shell
sudo snap install bulletty
```

[View on Snap Store](https://snapcraft.io/bulletty)

---

## 🍎 macOS

### Homebrew

**bulletty** is available on [Homebrew](https://brew.sh/):

```shell
brew install bulletty
```

[View formula on Homebrew](https://formulae.brew.sh/formula/bulletty)

---

## 🌐 Cross-platform

### Homebrew (Linux)

[Homebrew](https://brew.sh/) also works on Linux. If you have Homebrew installed on your Linux system:

```shell
brew install bulletty
```

### Through _Cargo_

If you have Rust and `cargo 1.90+` installed on any platform (Linux, macOS, or Windows):

```shell
cargo install bulletty
```

[bulletty on crates.io](https://crates.io/crates/bulletty)

---

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

### Notes on building on Windows

bulletty requires the `openssl` crate to build, but it's known to be a little complicated to build on Windows. If it fails on a regular build, it's probably because whatever `perl` version it's trying to use is not suitable to build it. In that case, try installing [Strawberry Perl](https://strawberryperl.com/) and make sure that `opensll` uses the one you just install to build:

```powershell
$env:OPENSSL_SRC_PERL = "C:\Strawberry\perl\bin\perl.exe"
```

Then `cargo build` should work normally.
