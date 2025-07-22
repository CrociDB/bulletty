<h1 align="center">📰 bulletty</h1>
<p align="center">The TUI RSS/ATOM feed reader that lets you decide where to store your data.</p>

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

## Philosophy

The whole idea is bringing back the syndicalized internet. You subscribe to the sources you like the most and you get their content whenever it's available. When you get it, it's local, it's yours. **bulletty** will generate a Markdown file of each entry from each source. You can read through the embedded reader, straight from your terminal, or using any text editor. Much like the posix philosophy, everything here is a file.

Registering a new feed is as easy as:


```shell
bulletty add https://crocidb.com/index.xml Programming
```

This will add the feed for my own blog into the **Programming** category. If no category is passed, it adds to **General**.

In the end, it will generate a structure of files like this:

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

All the needs to be done is to synchronize `$HOME/.local/share/bulletty/` to save your data, similarly to an Obsidian vault.

## Build

Clone the repository, then:

```shell
cargo build --release
```

## Contributing

I am very open for contributions to help make **bulletty** the best feed reader out there.

## License

Copyright (c) Bruno Croci

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
