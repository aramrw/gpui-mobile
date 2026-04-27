# GPUI Mobile

Mobile platform support for GPUI — iOS (Metal) and Android (Vulkan).

This fork provides the necessary platform implementations and packages to run GPUI applications on mobile devices. 
It also supports macOS as a desktop target.

## Usage

Add `gpui-mobile` to your `Cargo.toml`:

```toml
[dependencies]
gpui-mobile = { git = "https://github.com/your-aramrw/gpui-mobile", branch = "main" }
```

### Locking Revisions

To ensure stability against upstream GPUI changes, it is recommended to lock your dependencies to specific revisions in your `Cargo.toml`.

## Examples

Run the minimal example on macOS:

```bash
cargo run --example hello_world
```

## Platform Support

- **iOS**: Metal renderer, UIKit integration.
- **Android**: Vulkan renderer, NDK integration.
- **macOS**: Metal renderer, AppKit integration.

## Development

See `docs/` for architecture insights and platform-specific guides.
