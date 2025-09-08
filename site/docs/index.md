---
title: bulletty
summary: The TUI RSS/ATOM feed reader that lets you decide where to store your data.
show_datetime: false
sidebar_title: Home
---


<p align="center">
  <img src="img/screenshot.gif" alt="bulletty" />
</p>

**bulletty** is a TUI feed (RSS and ATOM) reader. Read your subscriptions within your terminal. It downloads the entries for offline reading so all the data is local and yours: your subscriptions, highlights, comments, etc. You only need to sync the `data` folder.

It's in active development.

## Features

 - [X] Subscribe to RSS and ATOM feed types
 - [X] All your data in one place
 - [X] Download entries
 - [X] Render Markdown
 - [ ] Themes
 - [ ] Read later
 - [ ] Highlight
 - [ ] Notes
 - [ ] Web view
 - [ ] Mouse support
 - [ ] Image support

## Install

The easiest way to install **bulletty** is through Cargo:

```shell
cargo install bulletty
```

### Adding new sources

For now, you can only add new feed sources through the CLI:

```shell
bulletty add https://crocidb.com/index.xml [Category]
```
**bulletty** will syncronize all your sources when you open the TUI, by just invoking `bulletty`. Another way to update every entry is through: 

```shell
bulletty update
```

More on the CLI commands with:

```shell
bulletty help
```

## Philosophy

The whole idea is bringing back the descentralized internet. You subscribe to the sources you like the most and you get their content whenever it's available. When you get it, it's local, it's yours. **bulletty** will generate a Markdown file of each entry from each source. You can read through the embedded reader, straight from your terminal, or using any text editor.

All your feed data will be at `$HOME/.local/share/bulletty/`, in this structure:

```shell
[~/.local/share/bulletty]$ tree
.
└── categories
    ├── Programming
    │   ├── bruno-croci
    │   │   ├── .feed.toml
    │   │   ├── about.md
    │   │   ├── demystifying-the-shebang-kernel-adventures.md
    │   │   ├── from-ides-to-the-terminal.md
    │   │   ├── i-wrote-a-webserver-in-haskell.md
    │   │   ├── ...
    ├── General
    │   ├── another-website
    │   │   ├── .feed.toml
    │   │   ├── some-post.md
    │   │   ├── ...

```

All the needs to be done is to synchronize the `bulletty` directory to save your data, similarly to an Obsidian vault.

## Build

```shell
git clone https://github.com/CrociDB/bulletty.git
cd bulletty
cargo build --release
```

## Contributing

I am very open for contributions to help make **bulletty** the best feed reader out there.

## License

Copyright (c) Bruno Croci

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
