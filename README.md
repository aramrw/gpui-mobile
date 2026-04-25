# GPUI Mobile

Mobile platform support for GPUI — iOS (Metal) and Android (Vulkan).

This fork provides the necessary platform implementations and packages to run GPUI applications on mobile devices. It also supports macOS as a desktop target.

## Usage

Add `gpui-mobile` to your `Cargo.toml`:

```toml
[dependencies]
gpui-mobile = { git = "https://github.com/your-username/gpui-mobile", branch = "main" }
```

### Zero-Ceremony Entry Point

Use the `#[gpui_mobile::main]` macro to automatically handle platform-specific initialization for macOS, iOS, and Android.

```rust
use gpui_mobile::gpui::{prelude::*, App, WindowOptions};

#[gpui_mobile::main]
fn main(cx: &mut App) {
    cx.open_window(WindowOptions::default(), |_, cx| {
        cx.new(|_| HelloWorld)
    });
}

struct HelloWorld;
impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().child("Hello, GPUI Mobile!")
    }
}
```

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

During development, you can use a local path dependency:

```toml
gpui-mobile = { path = "../gpui-mobile" }
```

See `docs/` for architecture insights and platform-specific guides.
