# Crate Documentation

**Version:** 0.1.0

**Format Version:** 43

# Module `bulletty`

## Modules

## Module `app`

```rust
pub mod app { /* ... */ }
```

### Types

#### Enum `AppWorkStatus`

```rust
pub enum AppWorkStatus {
    None,
    Working(f32, String),
}
```

##### Variants

###### `None`

###### `Working`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `f32` |  |
| 1 | `String` |  |

##### Implementations

###### Methods

- ```rust
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **WithSubscriber**
- **OwoColorize**
- **RefUnwindSafe**
- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

- **Sync**
- **ErasedDestructor**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Instrument**
#### Struct `App`

```rust
pub struct App {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn init(self: &mut Self, state: Box<dyn AppScreen>) { /* ... */ }
  ```

- ```rust
  pub fn run(self: &mut Self, terminal: DefaultTerminal) -> Result<()> { /* ... */ }
  ```

###### Trait Implementations

- **Instrument**
- **WithSubscriber**
- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

- **Sync**
- **OwoColorize**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **Send**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
## Module `cli`

```rust
pub mod cli { /* ... */ }
```

### Types

#### Struct `Cli`

**Attributes:**

- `#[command(name = "bulletty")]`
- `#[command(version, about = "Your TUI feed reader", long_about = None)]`

```rust
pub struct Cli {
    pub command: Option<Commands>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `command` | `Option<Commands>` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **OwoColorize**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Instrument**
- **ErasedDestructor**
- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

- **FromArgMatches**
  - ```rust
    fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error> { /* ... */ }
    ```

  - ```rust
    fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error> { /* ... */ }
    ```

  - ```rust
    fn update_from_arg_matches(self: &mut Self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error> { /* ... */ }
    ```

  - ```rust
    fn update_from_arg_matches_mut(self: &mut Self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **WithSubscriber**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Freeze**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **CommandFactory**
  - ```rust
    fn command<''b>() -> clap::Command { /* ... */ }
    ```

  - ```rust
    fn command_for_update<''b>() -> clap::Command { /* ... */ }
    ```

- **Args**
  - ```rust
    fn group_id() -> Option<clap::Id> { /* ... */ }
    ```

  - ```rust
    fn augment_args<''b>(__clap_app: clap::Command) -> clap::Command { /* ... */ }
    ```

  - ```rust
    fn augment_args_for_update<''b>(__clap_app: clap::Command) -> clap::Command { /* ... */ }
    ```

- **Parser**
#### Enum `Commands`

```rust
pub enum Commands {
    List,
    Add {
        url: String,
        category: Option<String>,
    },
    Update,
}
```

##### Variants

###### `List`

List all feeds and categories

###### `Add`

Add new feed

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `url` | `String` | The ATOM/RSS feed URL |
| `category` | `Option<String>` | The category to add under, if none is passed, it will be added to General |

###### `Update`

Update all feeds

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **WithSubscriber**
- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

- **FromArgMatches**
  - ```rust
    fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error> { /* ... */ }
    ```

  - ```rust
    fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error> { /* ... */ }
    ```

  - ```rust
    fn update_from_arg_matches(self: &mut Self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error> { /* ... */ }
    ```

  - ```rust
    fn update_from_arg_matches_mut<''b>(self: &mut Self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error> { /* ... */ }
    ```

- **IntoEither**
- **Send**
- **Freeze**
- **Unpin**
- **ErasedDestructor**
- **OwoColorize**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Subcommand**
  - ```rust
    fn augment_subcommands<''b>(__clap_app: clap::Command) -> clap::Command { /* ... */ }
    ```

  - ```rust
    fn augment_subcommands_for_update<''b>(__clap_app: clap::Command) -> clap::Command { /* ... */ }
    ```

  - ```rust
    fn has_subcommand(__clap_name: &str) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Instrument**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Functions

#### Function `run_main_cli`

```rust
pub fn run_main_cli(cli: Cli) -> color_eyre::Result<()> { /* ... */ }
```

## Module `core`

```rust
pub mod core { /* ... */ }
```

### Modules

## Module `defs`

```rust
pub mod defs { /* ... */ }
```

### Constants and Statics

#### Constant `CONFIG_PATH`

```rust
pub const CONFIG_PATH: &str = "bulletty";
```

#### Constant `CONFIG_FILE`

```rust
pub const CONFIG_FILE: &str = "config.toml";
```

#### Constant `DATA_DIR`

```rust
pub const DATA_DIR: &str = "bulletty";
```

#### Constant `DATA_CATEGORIES_DIR`

```rust
pub const DATA_CATEGORIES_DIR: &str = "categories";
```

#### Constant `DATA_CATEGORY_DEFAULT`

```rust
pub const DATA_CATEGORY_DEFAULT: &str = "General";
```

#### Constant `DATA_FEED`

```rust
pub const DATA_FEED: &str = ".feed.toml";
```

#### Constant `LOG_DIR`

```rust
pub const LOG_DIR: &str = "bulletty";
```

## Module `feed`

```rust
pub mod feed { /* ... */ }
```

### Modules

## Module `feedentry`

```rust
pub mod feedentry { /* ... */ }
```

### Types

#### Struct `FeedEntry`

```rust
pub struct FeedEntry {
    pub title: String,
    pub description: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub url: String,
    pub author: String,
    pub text: String,
    pub lastupdated: chrono::DateTime<chrono::Utc>,
    pub seen: bool,
    pub filepath: std::path::PathBuf,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `title` | `String` |  |
| `description` | `String` |  |
| `date` | `chrono::DateTime<chrono::Utc>` |  |
| `url` | `String` |  |
| `author` | `String` |  |
| `text` | `String` |  |
| `lastupdated` | `chrono::DateTime<chrono::Utc>` |  |
| `seen` | `bool` |  |
| `filepath` | `std::path::PathBuf` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Instrument**
- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DeserializeOwned**
- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **WithSubscriber**
- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> FeedEntry { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **OwoColorize**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> FeedEntry { /* ... */ }
    ```

- **ErasedDestructor**
## Module `feedparser`

```rust
pub mod feedparser { /* ... */ }
```

### Functions

#### Function `get_feed`

```rust
pub fn get_feed(url: &str) -> color_eyre::Result<crate::core::library::feeditem::FeedItem> { /* ... */ }
```

#### Function `get_feed_entries`

```rust
pub fn get_feed_entries(feed: &crate::core::library::feeditem::FeedItem) -> color_eyre::Result<Vec<crate::core::feed::feedentry::FeedEntry>> { /* ... */ }
```

#### Function `get_feed_entries_doc`

```rust
pub fn get_feed_entries_doc(doctxt: &str, defaultauthor: &str) -> color_eyre::Result<Vec<crate::core::feed::feedentry::FeedEntry>> { /* ... */ }
```

## Module `library`

```rust
pub mod library { /* ... */ }
```

### Modules

## Module `data`

```rust
pub mod data { /* ... */ }
```

### Modules

## Module `config`

```rust
pub mod config { /* ... */ }
```

### Types

#### Struct `Config`

```rust
pub struct Config {
    pub datapath: std::path::PathBuf,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `datapath` | `std::path::PathBuf` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **WithSubscriber**
- **OwoColorize**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **DeserializeOwned**
- **Send**
- **Freeze**
- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Instrument**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
## Module `librarydata`

```rust
pub mod librarydata { /* ... */ }
```

### Types

#### Struct `LibraryData`

```rust
pub struct LibraryData {
    pub path: std::path::PathBuf,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `path` | `std::path::PathBuf` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(datapath: &Path) -> LibraryData { /* ... */ }
  ```

- ```rust
  pub fn feed_exists(self: &Self, slug: &str, category: &str) -> bool { /* ... */ }
  ```

- ```rust
  pub fn delete_feed(self: &Self, slug: &str, category: &str) -> color_eyre::Result<()> { /* ... */ }
  ```

- ```rust
  pub fn feed_create(self: &Self, feed: &FeedItem) -> color_eyre::Result<()> { /* ... */ }
  ```

- ```rust
  pub fn generate_categories_tree(self: &Self) -> color_eyre::Result<Vec<FeedCategory>> { /* ... */ }
  ```

- ```rust
  pub fn load_feeds_from_category(self: &Self, category_name: &str, category: &Path) -> color_eyre::Result<Vec<FeedItem>> { /* ... */ }
  ```

- ```rust
  pub fn update_feed_entries(self: &Self, category: &FeedCategory, feed: &FeedItem, feedxml: Option<String>) -> color_eyre::Result<()> { /* ... */ }
  ```

- ```rust
  pub fn save_feed_entry(self: &Self, entry: &FeedEntry) -> color_eyre::Result<()> { /* ... */ }
  ```

- ```rust
  pub fn load_feed_entries(self: &Self, category: &FeedCategory, item: &FeedItem) -> color_eyre::Result<Vec<FeedEntry>> { /* ... */ }
  ```

- ```rust
  pub fn get_unread_feed(self: &Self, category: &str, feed_slug: &str) -> color_eyre::Result<u16> { /* ... */ }
  ```

- ```rust
  pub fn set_entry_seen(self: &Self, entry: &FeedEntry) { /* ... */ }
  ```

- ```rust
  pub fn toggle_entry_seen(self: &Self, entry: &FeedEntry) { /* ... */ }
  ```

###### Trait Implementations

- **Unpin**
- **RefUnwindSafe**
- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

- **OwoColorize**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Instrument**
- **WithSubscriber**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Freeze**
### Functions

#### Function `load_or_create`

```rust
pub fn load_or_create(path: &std::path::Path) { /* ... */ }
```

## Module `feedcategory`

```rust
pub mod feedcategory { /* ... */ }
```

### Types

#### Struct `FeedCategory`

```rust
pub struct FeedCategory {
    pub title: String,
    pub feeds: Vec<crate::core::library::feeditem::FeedItem>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `title` | `String` |  |
| `feeds` | `Vec<crate::core::library::feeditem::FeedItem>` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FeedCategory { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Instrument**
- **IntoEither**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **OwoColorize**
- **WithSubscriber**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Sync**
- **Freeze**
## Module `feeditem`

```rust
pub mod feeditem { /* ... */ }
```

### Types

#### Struct `FeedItem`

```rust
pub struct FeedItem {
    pub title: String,
    pub description: String,
    pub url: String,
    pub feed_url: String,
    pub author: String,
    pub slug: String,
    pub lastupdated: chrono::DateTime<chrono::Utc>,
    pub category: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `title` | `String` |  |
| `description` | `String` |  |
| `url` | `String` |  |
| `feed_url` | `String` |  |
| `author` | `String` |  |
| `slug` | `String` |  |
| `lastupdated` | `chrono::DateTime<chrono::Utc>` |  |
| `category` | `String` |  |

##### Implementations

###### Trait Implementations

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Send**
- **Sync**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **DeserializeOwned**
- **Unpin**
- **Instrument**
- **IntoEither**
- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> FeedItem { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> FeedItem { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **WithSubscriber**
- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **OwoColorize**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Module `feedlibrary`

```rust
pub mod feedlibrary { /* ... */ }
```

### Types

#### Struct `FeedLibrary`

```rust
pub struct FeedLibrary {
    pub feedcategories: Vec<crate::core::library::feedcategory::FeedCategory>,
    pub data: crate::core::library::data::librarydata::LibraryData,
    pub updater: Option<crate::core::library::updater::Updater>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `feedcategories` | `Vec<crate::core::library::feedcategory::FeedCategory>` |  |
| `data` | `crate::core::library::data::librarydata::LibraryData` |  |
| `updater` | `Option<crate::core::library::updater::Updater>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn add_feed_from_url(self: &mut Self, url: &str, category: &Option<String>) -> color_eyre::Result<FeedItem> { /* ... */ }
  ```

- ```rust
  pub fn add_feed(self: &mut Self, feed: FeedItem) -> color_eyre::Result<FeedItem> { /* ... */ }
  ```

- ```rust
  pub fn delete_feed(self: &Self, slug: &str, category: &str) -> color_eyre::Result<()> { /* ... */ }
  ```

- ```rust
  pub fn get_feed_entries_by_category(self: &Self, categorytitle: &str) -> Vec<FeedEntry> { /* ... */ }
  ```

- ```rust
  pub fn get_feed_entries_by_item_slug(self: &Self, slug: &str) -> Vec<FeedEntry> { /* ... */ }
  ```

- ```rust
  pub fn start_updater(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn update(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn get_update_status(self: &Self) -> AppWorkStatus { /* ... */ }
  ```

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **WithSubscriber**
- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **OwoColorize**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Instrument**
- **Send**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
## Module `updater`

```rust
pub mod updater { /* ... */ }
```

### Types

#### Struct `Updater`

```rust
pub struct Updater {
    pub last_completed: std::sync::Arc<std::sync::Mutex<String>>,
    pub total_completed: std::sync::Arc<std::sync::atomic::AtomicU16>,
    pub finished: std::sync::Arc<std::sync::atomic::AtomicBool>,
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `last_completed` | `std::sync::Arc<std::sync::Mutex<String>>` |  |
| `total_completed` | `std::sync::Arc<std::sync::atomic::AtomicU16>` |  |
| `finished` | `std::sync::Arc<std::sync::atomic::AtomicBool>` |  |
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new(feedcategories: Vec<FeedCategory>) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Instrument**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **WithSubscriber**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **ErasedDestructor**
- **Freeze**
- **OwoColorize**
- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
## Module `ui`

```rust
pub mod ui { /* ... */ }
```

### Modules

## Module `appscreen`

```rust
pub mod appscreen { /* ... */ }
```

### Types

#### Enum `AppScreenEvent`

```rust
pub enum AppScreenEvent {
    None,
    ChangeState(Box<dyn AppScreen>),
    ExitState,
    OpenDialog(Box<dyn Dialog>),
    CloseDialog,
    ExitApp,
}
```

##### Variants

###### `None`

###### `ChangeState`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<dyn AppScreen>` |  |

###### `ExitState`

###### `OpenDialog`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<dyn Dialog>` |  |

###### `CloseDialog`

###### `ExitApp`

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **OwoColorize**
- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **WithSubscriber**
- **Instrument**
- **ErasedDestructor**
- **Freeze**
### Traits

#### Trait `AppScreen`

```rust
pub trait AppScreen {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `start`
- `quit`
- `pause`
- `unpause`
- `render`
- `handle_events`
- `handle_keypress`
- `get_work_status`
- `get_title`
- `get_instructions`
- `get_full_instructions`

##### Implementations

This trait is implemented for the following types:

- `HelpDialog`
- `MainScreen`
- `ReaderScreen`

## Module `dialog`

```rust
pub mod dialog { /* ... */ }
```

### Traits

#### Trait `Dialog`

```rust
pub trait Dialog {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `get_size`: Get the sizes of dialog
- `as_screen`
- `as_screen_mut`

##### Implementations

This trait is implemented for the following types:

- `HelpDialog`

## Module `logging`

```rust
pub mod logging { /* ... */ }
```

### Functions

#### Function `init`

```rust
pub fn init() -> Option<tracing_appender::non_blocking::WorkerGuard> { /* ... */ }
```

## Module `mainui`

```rust
pub mod mainui { /* ... */ }
```

### Functions

#### Function `run_main_ui`

```rust
pub fn run_main_ui() -> color_eyre::Result<()> { /* ... */ }
```

## Module `ui`

```rust
pub mod ui { /* ... */ }
```

### Modules

## Module `screens`

```rust
pub mod screens { /* ... */ }
```

### Modules

## Module `helpdialog`

```rust
pub mod helpdialog { /* ... */ }
```

### Types

#### Struct `HelpDialog`

```rust
pub struct HelpDialog {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new(help_string: String) -> HelpDialog { /* ... */ }
  ```

###### Trait Implementations

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **WithSubscriber**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **AppScreen**
  - ```rust
    fn start(self: &mut Self) { /* ... */ }
    ```

  - ```rust
    fn quit(self: &mut Self) { /* ... */ }
    ```

  - ```rust
    fn pause(self: &mut Self) { /* ... */ }
    ```

  - ```rust
    fn unpause(self: &mut Self) { /* ... */ }
    ```

  - ```rust
    fn render(self: &mut Self, frame: &mut ratatui::Frame<''_>, area: ratatui::prelude::Rect) { /* ... */ }
    ```

  - ```rust
    fn handle_events(self: &mut Self) -> Result<AppScreenEvent> { /* ... */ }
    ```

  - ```rust
    fn handle_keypress(self: &mut Self, key: KeyEvent) -> Result<AppScreenEvent> { /* ... */ }
    ```

  - ```rust
    fn get_work_status(self: &Self) -> AppWorkStatus { /* ... */ }
    ```

  - ```rust
    fn get_title(self: &Self) -> String { /* ... */ }
    ```

  - ```rust
    fn get_instructions(self: &Self) -> String { /* ... */ }
    ```

  - ```rust
    fn get_full_instructions(self: &Self) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Freeze**
- **OwoColorize**
- **RefUnwindSafe**
- **IntoEither**
- **Dialog**
  - ```rust
    fn get_size(self: &Self) -> ratatui::prelude::Rect { /* ... */ }
    ```

  - ```rust
    fn as_screen(self: &Self) -> &dyn AppScreen { /* ... */ }
    ```

  - ```rust
    fn as_screen_mut(self: &mut Self) -> &mut dyn AppScreen { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Instrument**
## Module `mainscreen`

```rust
pub mod mainscreen { /* ... */ }
```

### Types

#### Struct `MainScreen`

```rust
pub struct MainScreen {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **OwoColorize**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **AppScreen**
  - ```rust
    fn start(self: &mut Self) { /* ... */ }
    ```

  - ```rust
    fn quit(self: &mut Self) { /* ... */ }
    ```

  - ```rust
    fn pause(self: &mut Self) { /* ... */ }
    ```

  - ```rust
    fn unpause(self: &mut Self) { /* ... */ }
    ```

  - ```rust
    fn render(self: &mut Self, frame: &mut ratatui::Frame<''_>, area: Rect) { /* ... */ }
    ```

  - ```rust
    fn handle_events(self: &mut Self) -> Result<AppScreenEvent> { /* ... */ }
    ```

  - ```rust
    fn handle_keypress(self: &mut Self, key: crossterm::event::KeyEvent) -> Result<AppScreenEvent> { /* ... */ }
    ```

  - ```rust
    fn get_title(self: &Self) -> String { /* ... */ }
    ```

  - ```rust
    fn get_instructions(self: &Self) -> String { /* ... */ }
    ```

  - ```rust
    fn get_work_status(self: &Self) -> AppWorkStatus { /* ... */ }
    ```

  - ```rust
    fn get_full_instructions(self: &Self) -> String { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Instrument**
- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **IntoEither**
- **WithSubscriber**
## Module `readerscreen`

```rust
pub mod readerscreen { /* ... */ }
```

### Types

#### Struct `ReaderScreen`

```rust
pub struct ReaderScreen {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new(entry: FeedEntry) -> ReaderScreen { /* ... */ }
  ```

- ```rust
  pub fn scrollup(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn scrolldown(self: &mut Self) { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Instrument**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **WithSubscriber**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **ErasedDestructor**
- **OwoColorize**
- **AppScreen**
  - ```rust
    fn start(self: &mut Self) { /* ... */ }
    ```

  - ```rust
    fn render(self: &mut Self, frame: &mut ratatui::Frame<''_>, area: ratatui::prelude::Rect) { /* ... */ }
    ```

  - ```rust
    fn handle_events(self: &mut Self) -> Result<AppScreenEvent> { /* ... */ }
    ```

  - ```rust
    fn handle_keypress(self: &mut Self, key: crossterm::event::KeyEvent) -> color_eyre::eyre::Result<AppScreenEvent> { /* ... */ }
    ```

  - ```rust
    fn pause(self: &mut Self) { /* ... */ }
    ```

  - ```rust
    fn unpause(self: &mut Self) { /* ... */ }
    ```

  - ```rust
    fn quit(self: &mut Self) { /* ... */ }
    ```

  - ```rust
    fn get_title(self: &Self) -> String { /* ... */ }
    ```

  - ```rust
    fn get_instructions(self: &Self) -> String { /* ... */ }
    ```

  - ```rust
    fn get_work_status(self: &Self) -> AppWorkStatus { /* ... */ }
    ```

  - ```rust
    fn get_full_instructions(self: &Self) -> String { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

## Module `states`

```rust
pub mod states { /* ... */ }
```

### Modules

## Module `feedentrystate`

```rust
pub mod feedentrystate { /* ... */ }
```

### Types

#### Struct `FeedEntryState`

```rust
pub struct FeedEntryState {
    pub entries: Vec<crate::core::feed::feedentry::FeedEntry>,
    pub listatate: ratatui::widgets::ListState,
    pub previous_selected: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `entries` | `Vec<crate::core::feed::feedentry::FeedEntry>` |  |
| `listatate` | `ratatui::widgets::ListState` |  |
| `previous_selected` | `String` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn update(self: &mut Self, library: &FeedLibrary, treestate: &FeedTreeState) { /* ... */ }
  ```

- ```rust
  pub fn get_items(self: &Self) -> Vec<ListItem<''_>> { /* ... */ }
  ```

- ```rust
  pub fn get_selected(self: &Self) -> Option<FeedEntry> { /* ... */ }
  ```

- ```rust
  pub fn set_current_read(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn select_next(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn select_previous(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn select_first(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn select_last(self: &mut Self) { /* ... */ }
  ```

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Instrument**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **OwoColorize**
- **ErasedDestructor**
- **Send**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **WithSubscriber**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Module `feedtreestate`

```rust
pub mod feedtreestate { /* ... */ }
```

### Types

#### Enum `FeedItemInfo`

```rust
pub enum FeedItemInfo {
    Category(String),
    Item(String, String, String),
}
```

##### Variants

###### `Category`

Represents the category title

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Item`

Represents an item in the feed tree with a title, categore, and slug

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |
| 1 | `String` |  |
| 2 | `String` |  |

##### Implementations

###### Trait Implementations

- **WithSubscriber**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **OwoColorize**
- **Send**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Instrument**
- **ErasedDestructor**
- **Sync**
#### Struct `FeedTreeState`

```rust
pub struct FeedTreeState {
    pub treeitems: Vec<FeedItemInfo>,
    pub listatate: ratatui::widgets::ListState,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `treeitems` | `Vec<FeedItemInfo>` |  |
| `listatate` | `ratatui::widgets::ListState` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn update(self: &mut Self, library: &FeedLibrary) { /* ... */ }
  ```

- ```rust
  pub fn get_items(self: &Self, library: &FeedLibrary) -> Vec<ListItem<''_>> { /* ... */ }
  ```

- ```rust
  pub fn get_selected(self: &Self) -> Option<&FeedItemInfo> { /* ... */ }
  ```

- ```rust
  pub fn select_next(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn select_previous(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn select_first(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn select_last(self: &mut Self) { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **OwoColorize**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **Send**
- **Sync**
- **Freeze**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Instrument**
- **Unpin**
- **WithSubscriber**
- **PolicyExt**
  - ```rust
    fn and<P, B, E>(self: Self, other: P) -> And<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

  - ```rust
    fn or<P, B, E>(self: Self, other: P) -> Or<T, P>
where
    T: Policy<B, E>,
    P: Policy<B, E> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Functions

### Function `run`

```rust
pub fn run() -> color_eyre::Result<()> { /* ... */ }
```

