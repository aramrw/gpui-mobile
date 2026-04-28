use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;

    // If the user named their function 'main', we MUST rename it internally
    // to avoid conflict with the generated 'fn main()' on macOS.
    let user_fn_name = if name == "main" {
        quote::format_ident!("__gpui_user_main")
    } else {
        name.clone()
    };

    let expanded = quote! {
        #(#attrs)*
        #vis fn #user_fn_name(cx: &mut gpui_mobile::gpui::App) {
            #body
        }

        #[cfg(target_os = "macos")]
        fn main() {
            let platform = gpui_mobile::macos::MacPlatform::new(false);
            gpui_mobile::gpui::Application::with_platform(std::rc::Rc::new(platform)).run(|cx| {
                #user_fn_name(cx);
            });
        }

        #[cfg(target_os = "ios")]
        #[unsafe(no_mangle)]
        pub extern "C" fn gpui_ios_register_app() {
            gpui_mobile::ios::ffi::set_app_callback(Box::new(|cx| {
                #user_fn_name(cx);
            }));
        }

        #[cfg(target_os = "android")]
        #[unsafe(no_mangle)]
        fn android_main(app: gpui_mobile::android::jni::AndroidApp) {
             gpui_mobile::android::jni::init_platform(&app);
             gpui_mobile::gpui::Application::new().run(|cx| {
                #user_fn_name(cx);
             });
        }

        // Dummy main to satisfy `cargo` when compiling a bin crate on iOS/Android
        #[cfg(not(target_os = "macos"))]
        fn main() {}
    };

    TokenStream::from(expanded)
}
