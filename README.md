[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit)

[![cargo.io](https://img.shields.io/crates/v/clipboard-win-html)](https://crates.io/crates/clipboard-win-html)
![License](https://img.shields.io/crates/l/clipboard-win-html?color=purple)

[![rust badge](https://img.shields.io/static/v1?label=Made%20with&message=Rust&style=for-the-badge&logo=rust&labelColor=e82833&color=b11522)](https://www.rust-lang.org/)

# Description

Copy HTML to Windows clipboard.

# Installation

Add this line to your dependencies section of your `Cargo.toml` file.

`clipboard-win-html = "0.1"`

# Usage

```rust
use clipboard_win_html::set_clipboard_html;

set_clipboard_html("<h1>Pure, valid, HTML.</h1>");
```

Paste away.

# Contributing

If there are any features you would like added, found any potential bugs, or have any questions, then feel free to create an issue.

## Testing

`cargo test`

Unittests are in the same file, next to the units they are testing (bottom). Integration tests are in `/tests/`.

# License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
