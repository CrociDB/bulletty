---
title: CLI Reference
summary: The CLI commands reference for bulletty
show_datetime: false
---

**bulletty** has the following commands:

 - `list`: List all feeds and categories
 - `add`: Add new feed
 - `update`: Update all feeds
 - `delete`: Delete a feed
 - `dirs`: Show important directories
 - `import`: Import a list of feed sources through OPML
 - `export`: Export all your sources to an OPML file
 - `help`: Print this message or the help of the given subcommand(s)

## list

Lists all the categories and feeds on each of them, along with their _slugs_, the slugified name without spaces that is used as a directory in the filesystem.

## add *feed_url* [*category*]

Adds a new feed source to the specified category. If no category is specified, it's added to `General`.

## update

This will check for new articles in all of the feeds registered.

## delete [*feed name* / *url* / *slug*]

It will find the feed with the specified name/url/slug and prompt you to delete it with all of its articles.

## dirs

Displays important directories used by **bulletty**, including the library directory, that can be synchronized with your other machiens.

## import [*opml file*]

Imports a list of feed sources form an OPML file. Other feed readers usually generate these types of files.

## export [*opml file*]

Exports the list of feed source to an OPML file.

## help

Display all the commands and their description.
