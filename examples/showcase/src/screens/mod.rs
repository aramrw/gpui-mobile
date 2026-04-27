pub mod about;
pub mod audio_player;
pub mod chat;
pub mod components;
pub mod counter;
pub mod feed;
pub mod form;
pub mod home;
pub mod packages_demo;
pub mod settings;
pub mod swiper;
pub mod video_player;
pub mod webview_browser;

use crate::demos::{AnimationPlayground, ShaderShowcase};
use gpui::{
    div, point, prelude::*, px, rgb, size, Bounds, Context, Entity, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, SharedString, Window,
};
use gpui_mobile::components::material::{MaterialTheme, NavigationBarBuilder, TopAppBar};
use gpui_mobile::{set_system_chrome, StatusBarContentStyle, SystemChromeStyle};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Home,
    Settings,
    About,
    Counter,
    AppleGlass,
    Material,
    Form,
    Animations,
    Shaders,
    PackagesDemo,
    WebViewBrowser,
    Swiper,
    Feed,
    Chat,
    AudioPlayer,
    VideoPlayer,
}

impl Screen {
    pub fn from_deeplink_url(url: &str) -> Option<Self> {
        let stripped = url
            .strip_prefix("gpui://")
            .or_else(|| url.strip_prefix("gpui:"))?;
        let path = stripped.split('/').next().unwrap_or("").trim();
        if path.is_empty() {
            return None;
        }
        match path.to_ascii_lowercase().as_str() {
            "settings" => Some(Screen::Settings),
            "about" => Some(Screen::About),
            "home" => Some(Screen::Home),
            "counter" => Some(Screen::Counter),
            "apple_glass" | "appleglass" => Some(Screen::AppleGlass),
            "material" => Some(Screen::Material),
            "form" => Some(Screen::Form),
            "animations" => Some(Screen::Animations),
            "shaders" => Some(Screen::Shaders),
            "packages" | "packages_demo" => Some(Screen::PackagesDemo),
            "webview" | "webview_browser" | "browser" => Some(Screen::WebViewBrowser),
            "swiper" | "discover" => Some(Screen::Swiper),
            "feed" => Some(Screen::Feed),
            "chat" => Some(Screen::Chat),
            "audio_player" | "audio" => Some(Screen::AudioPlayer),
            "video_player" | "video" => Some(Screen::VideoPlayer),
            _ => None,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Screen::Settings => "Settings",
            Screen::About => "About",
            Screen::Home => "Home",
            Screen::Counter => "Counter",
            Screen::AppleGlass => "Apple Liquid Glass",
            Screen::Material => "Material Design 3",
            Screen::Form => "Material Form",
            Screen::Animations => "Animations",
            Screen::Shaders => "Shaders",
            Screen::PackagesDemo => "Packages",
            Screen::WebViewBrowser => "Browser",
            Screen::Swiper => "Discover",
            Screen::Feed => "Feed",
            Screen::Chat => "Sarah Johnson",
            Screen::AudioPlayer => "Audio Player",
            Screen::VideoPlayer => "Video Player",
        }
    }

    pub fn is_tab_root(&self) -> bool {
        matches!(
            self,
            Screen::Home | Screen::Counter | Screen::Settings | Screen::About
        )
    }
}

pub const BASE: u32 = 0x121318;
pub const SURFACE0: u32 = 0x1E1F25;
pub const SURFACE1: u32 = 0x282A2F;
pub const TEXT: u32 = 0xE2E2E9;
pub const SUBTEXT: u32 = 0xC4C6D0;
pub const BLUE: u32 = 0x4285F4;
pub const GREEN: u32 = 0x34A853;
pub const RED: u32 = 0xEA4335;
pub const MAUVE: u32 = 0xA142F4;
pub const YELLOW: u32 = 0xFBBC04;
pub const PEACH: u32 = 0xFA7B17;
pub const TEAL: u32 = 0x24C1E0;
pub const MANTLE: u32 = 0x0D0E13;
pub const SKY: u32 = 0x4FC3F7;
pub const LAVENDER: u32 = 0x7B8CF8;

pub const LIGHT_TEXT: u32 = 0x1A1B20;
pub const LIGHT_SUBTEXT: u32 = 0x44474F;
pub const LIGHT_CARD_BG: u32 = 0xEDEDF4;
pub const LIGHT_DIVIDER: u32 = 0xC4C6D0;

#[derive(Debug, Clone, Copy, Default)]
pub struct SafeArea {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

pub struct Router {
    pub current_screen: Screen,
    pub tap_count: u32,
    pub user_name: SharedString,
    pub dark_mode: bool,
    pub font_size_multiplier: f32,
    history: Vec<Screen>,
    pub safe_area: SafeArea,

    zoom_header: Entity<gpui_mobile::components::material::ZoomHeader>,

    animation_playground: Option<AnimationPlayground>,
    shader_showcase: Option<ShaderShowcase>,
}

impl Router {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self::with_initial_screen(Screen::default(), window, cx)
    }

    pub fn with_initial_screen(
        screen: Screen,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let safe_area = Self::query_safe_area();

        let user_name = if cfg!(target_os = "ios") {
            "iOS"
        } else if cfg!(target_os = "android") {
            "Android"
        } else {
            "Mobile"
        };

        let mut history = Vec::new();
        if !screen.is_tab_root() {
            history.push(Screen::Home);
        }

        let zoom_header = cx.new(|_cx| {
            gpui_mobile::components::material::ZoomHeader::new(MaterialTheme::from_appearance(true))
        });

        Self {
            current_screen: screen,
            tap_count: 0,
            user_name: user_name.into(),
            dark_mode: true,
            font_size_multiplier: 1.2,
            history,
            safe_area,
            zoom_header,
            animation_playground: None,
            shader_showcase: None,
        }
    }

    fn query_safe_area() -> SafeArea {
        #[cfg(target_os = "android")]
        {
            use gpui_mobile::android::jni;
            if let Some(platform) = jni::platform() {
                if let Some(win) = platform.primary_window() {
                    let insets = win.safe_area_insets_logical();
                    log::info!(
                        "Router: safe area insets (logical px): top={:.1} bottom={:.1} left={:.1} right={:.1}",
                        insets.top, insets.bottom, insets.left, insets.right,
                    );
                    return SafeArea {
                        top: insets.top,
                        bottom: insets.bottom,
                        left: insets.left,
                        right: insets.right,
                    };
                }
            }
        }

        #[cfg(target_os = "ios")]
        {
            let (top, bottom, left, right) = gpui_mobile::safe_area_insets();
            if top > 0.0 || bottom > 0.0 {
                return SafeArea {
                    top,
                    bottom,
                    left,
                    right,
                };
            }
            return SafeArea {
                top: 59.0,
                bottom: 34.0,
                left: 0.0,
                right: 0.0,
            };
        }

        #[allow(unreachable_code)]
        SafeArea::default()
    }

    pub fn navigate_to(&mut self, screen: Screen) {
        if self.current_screen != screen {
            let _ = gpui_mobile::packages::vibration::haptic_feedback(
                gpui_mobile::packages::vibration::HapticFeedback::Selection,
            );
            if self.current_screen == Screen::AudioPlayer {
                audio_player::dismiss();
            }
            form::dismiss_form_keyboard();
            chat::dismiss_chat();
            if screen.is_tab_root() {
                self.history.clear();
            } else {
                self.history.push(self.current_screen);
            }
            self.current_screen = screen;

            match screen {
                Screen::Animations if self.animation_playground.is_none() => {
                    self.animation_playground = Some(AnimationPlayground::new());
                }
                Screen::Shaders if self.shader_showcase.is_none() => {
                    self.shader_showcase = Some(ShaderShowcase::new());
                }
                _ => {}
            }
        }
    }

    pub fn go_back(&mut self) -> bool {
        if let Some(prev) = self.history.pop() {
            let _ = gpui_mobile::packages::vibration::haptic_feedback(
                gpui_mobile::packages::vibration::HapticFeedback::Selection,
            );
            if self.current_screen == Screen::AudioPlayer {
                audio_player::dismiss();
            }
            form::dismiss_form_keyboard();
            chat::dismiss_chat();
            self.current_screen = prev;
            true
        } else {
            false
        }
    }

    pub fn can_go_back(&self) -> bool {
        !self.current_screen.is_tab_root() && !self.history.is_empty()
    }
}

impl Render for Router {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        window.set_rem_size(px(16.0 * self.font_size_multiplier));

        let show_tab_bar = self.current_screen.is_tab_root();
        let theme =
            gpui_mobile::components::material::MaterialTheme::from_appearance(self.dark_mode);

        let _ = self.zoom_header.update(cx, |this, cx| {
            this.set_theme(theme, cx);
        });

        let bg_color = theme.surface;
        let text_color = theme.on_surface;
        let safe_top = self.safe_area.top;
        let safe_bottom = self.safe_area.bottom;

        let chrome = self.system_chrome_style();
        let top_color = chrome.status_bar_color.unwrap_or(bg_color);
        let bottom_color = chrome.navigation_bar_color.unwrap_or(bg_color);

        set_system_chrome(&chrome);

        let is_fullscreen_demo =
            matches!(self.current_screen, Screen::Animations | Screen::Shaders);

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(bg_color))
            .text_color(rgb(text_color))
            .when(safe_top > 0.0, |d| {
                d.child(div().w_full().h(px(safe_top)).bg(rgb(top_color)))
            })
            .when(cfg!(target_os = "ios") && !is_fullscreen_demo, |d| {
                d.child(self.render_top_bar(cx))
            })
            .child(self.render_current_screen(window, cx))
            .when(show_tab_bar, |d| d.child(self.render_tab_bar(cx)))
            .when(safe_bottom > 0.0 && show_tab_bar, |d| {
                d.child(div().w_full().h(px(safe_bottom)).bg(rgb(bottom_color)))
            })
            .child(self.zoom_header.clone())
            .into_any_element()
    }
}

impl Router {
    fn system_chrome_style(&self) -> SystemChromeStyle {
        let is_fullscreen_demo =
            matches!(self.current_screen, Screen::Animations | Screen::Shaders);
        let theme =
            gpui_mobile::components::material::MaterialTheme::from_appearance(self.dark_mode);

        if is_fullscreen_demo {
            SystemChromeStyle {
                status_bar_color: Some(BASE),
                status_bar_style: StatusBarContentStyle::Light,
                navigation_bar_color: Some(BASE),
            }
        } else {
            SystemChromeStyle {
                status_bar_color: Some(theme.surface),
                status_bar_style: if self.dark_mode {
                    StatusBarContentStyle::Light
                } else {
                    StatusBarContentStyle::Dark
                },
                navigation_bar_color: Some(if self.current_screen.is_tab_root() {
                    theme.surface_container
                } else {
                    theme.surface
                }),
            }
        }
    }

    fn render_current_screen(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        match self.current_screen {
            Screen::Animations => {
                return self
                    .render_animations_content(window, cx)
                    .into_any_element();
            }
            Screen::Shaders => {
                return self.render_shaders_content(window, cx).into_any_element();
            }
            _ => {}
        }

        let screen_content = match self.current_screen {
            Screen::Home => self.render_home_screen(cx).into_any_element(),
            Screen::Settings => self.render_settings_screen(cx).into_any_element(),
            Screen::About => self.render_about_screen(cx).into_any_element(),
            Screen::Counter => self.render_counter_screen(cx).into_any_element(),
            Screen::AppleGlass => self.render_apple_glass_screen(cx).into_any_element(),
            Screen::Material => self.render_material_screen(cx).into_any_element(),
            Screen::Form => self.render_form_screen(cx).into_any_element(),
            Screen::PackagesDemo => self.render_packages_demo_screen(cx).into_any_element(),
            Screen::WebViewBrowser => self.render_webview_browser_screen(cx).into_any_element(),
            Screen::Swiper => self.render_swiper_screen(cx).into_any_element(),
            Screen::Feed => self.render_feed_screen(cx).into_any_element(),
            Screen::Chat => self.render_chat_screen(cx).into_any_element(),
            Screen::AudioPlayer => self.render_audio_player_screen(cx).into_any_element(),
            Screen::VideoPlayer => self.render_video_player_screen(window, cx).into_any_element(),
            _ => div().child("Not implemented").into_any_element(),
        };

        div()
            .id("screen-scroll-container")
            .flex_1()
            .overflow_y_scroll()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|_this, _event: &MouseDownEvent, _window, cx| {
                    let form_had_focus = form::has_focused_field();
                    let chat_had_focus = chat::CHAT_STATE.with(|s: &std::cell::RefCell<chat::ChatState>| s.borrow().focused);
                    if form_had_focus {
                        form::dismiss_form_keyboard();
                    }
                    if chat_had_focus {
                        chat::dismiss_chat();
                    }
                    if form_had_focus || chat_had_focus {
                        cx.notify();
                    }
                }),
            )
            .child(screen_content)
            .into_any_element()
    }

    fn render_tab_bar(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let current = self.current_screen;
        let dark = self.dark_mode;

        NavigationBarBuilder::new(dark)
            .item(
                "⌂",
                "Home",
                current == Screen::Home,
                cx.listener(move |this, _, _, cx| {
                    this.navigate_to(Screen::Home);
                    cx.notify();
                }),
            )
            .item(
                "±",
                "Counter",
                current == Screen::Counter,
                cx.listener(move |this, _, _, cx| {
                    this.navigate_to(Screen::Counter);
                    cx.notify();
                }),
            )
            .item(
                "☰",
                "Settings",
                current == Screen::Settings,
                cx.listener(move |this, _, _, cx| {
                    this.navigate_to(Screen::Settings);
                    cx.notify();
                }),
            )
            .item(
                "♥",
                "About",
                current == Screen::About,
                cx.listener(move |this, _, _, cx| {
                    this.navigate_to(Screen::About);
                    cx.notify();
                }),
            )
            .build()
    }

    fn render_home_screen(&self, cx: &mut Context<Self>) -> impl IntoElement {
        home::render(self, cx)
    }

    fn render_settings_screen(&self, cx: &mut Context<Self>) -> impl IntoElement {
        settings::render(self, cx)
    }

    fn render_about_screen(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        about::render(self)
    }

    fn render_counter_screen(&self, cx: &mut Context<Self>) -> impl IntoElement {
        counter::render(self, cx)
    }
    
    fn render_apple_glass_screen(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        components::render_apple_glass(self)
    }

    fn render_material_screen(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        components::render_material(self)
    }

    fn render_form_screen(&self, cx: &mut Context<Self>) -> impl IntoElement {
        form::render(self, cx)
    }

    fn render_packages_demo_screen(&self, cx: &mut Context<Self>) -> impl IntoElement {
        packages_demo::render(self, cx)
    }

    fn render_webview_browser_screen(&self, cx: &mut Context<Self>) -> impl IntoElement {
        webview_browser::render(self, cx)
    }

    fn render_swiper_screen(&self, cx: &mut Context<Self>) -> impl IntoElement {
        swiper::render(self, cx)
    }

    fn render_feed_screen(&self, cx: &mut Context<Self>) -> impl IntoElement {
        feed::render(self, cx)
    }

    fn render_chat_screen(&self, cx: &mut Context<Self>) -> impl IntoElement {
        chat::render(self, cx)
    }

    fn render_audio_player_screen(&self, cx: &mut Context<Self>) -> impl IntoElement {
        audio_player::render(self, cx)
    }

    fn render_video_player_screen(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        video_player::render(self, window, cx)
    }

    fn render_top_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme =
            gpui_mobile::components::material::MaterialTheme::from_appearance(self.dark_mode);
        let title = self.current_screen.title();

        let mut bar = TopAppBar::center_aligned(title, theme);

        if self.can_go_back() {
            bar = bar.leading_icon("←", cx.listener(|this, _, _, cx| {
                this.go_back();
                cx.notify();
            }));
        }

        bar.build()
    }

    fn render_animations_content(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        window.request_animation_frame();

        if self.animation_playground.is_none() {
            self.animation_playground = Some(AnimationPlayground::new());
        }

        let viewport = window.viewport_size();
        if let Some(playground) = &mut self.animation_playground {
            playground.set_bounds(Bounds {
                origin: point(0.0, 0.0),
                size: size(viewport.width.as_f32(), viewport.height.as_f32()),
            });
        }

        div()
            .flex_1()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _window, cx| {
                    if let Some(playground) = &mut this.animation_playground {
                        let pos = point(event.position.x.as_f32(), event.position.y.as_f32());
                        playground.touch_start = Some((pos, std::time::Instant::now()));
                        playground.current_touch = Some(pos);
                        cx.notify();
                    }
                }),
            )
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                if let Some(playground) = &mut this.animation_playground {
                    let pos = point(event.position.x.as_f32(), event.position.y.as_f32());
                    if playground.touch_start.is_none() {
                        playground.touch_start = Some((pos, std::time::Instant::now()));
                    }
                    playground.current_touch = Some(pos);
                    cx.notify();
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, event: &MouseUpEvent, _window, cx| {
                    if let Some(playground) = &mut this.animation_playground {
                        let position = point(event.position.x.as_f32(), event.position.y.as_f32());

                        if let Some((start_pos, start_time)) = playground.touch_start.take() {
                            let elapsed = start_time.elapsed();
                            let dx = position.x - start_pos.x;
                            let dy = position.y - start_pos.y;
                            let distance = (dx * dx + dy * dy).sqrt();

                            if elapsed < std::time::Duration::from_millis(200) && distance < 20.0 {
                                let color_rgb = crate::demos::random_color(playground.next_ball_id);
                                playground.spawn_particles(position, rgb(color_rgb).into());
                                playground.next_ball_id += 1;
                            } else {
                                let dt = elapsed.as_secs_f32().max(0.01);
                                let velocity = point(dx / dt * 0.5, dy / dt * 0.5);
                                playground.spawn_ball(start_pos, velocity);
                            }
                        }
                        playground.current_touch = None;
                        cx.notify();
                    }
                }),
            )
            .child(if let Some(playground) = &mut self.animation_playground {
                playground.render_content(window).into_any_element()
            } else {
                div().into_any_element()
            })
    }

    fn render_shaders_content(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        window.request_animation_frame();

        if self.shader_showcase.is_none() {
            self.shader_showcase = Some(ShaderShowcase::new());
        }

        if let Some(showcase) = &mut self.shader_showcase {
            let viewport = window.viewport_size();
            showcase.set_screen_center(point(
                viewport.width.as_f32() / 2.0,
                viewport.height.as_f32() / 2.0,
            ));
        }

        div()
            .flex_1()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _window, cx| {
                    if let Some(showcase) = &mut this.shader_showcase {
                        let pos = point(event.position.x.as_f32(), event.position.y.as_f32());
                        showcase.touch_position = Some(pos);
                        showcase.spawn_ripple(pos);
                        cx.notify();
                    }
                }),
            )
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                if let Some(showcase) = &mut this.shader_showcase {
                    let pos = point(event.position.x.as_f32(), event.position.y.as_f32());
                    showcase.touch_position = Some(pos);
                    cx.notify();
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _event: &MouseUpEvent, _window, cx| {
                    if let Some(showcase) = &mut this.shader_showcase {
                        showcase.touch_position = None;
                        cx.notify();
                    }
                }),
            )
            .child(if let Some(showcase) = &mut self.shader_showcase {
                showcase.render_content(window).into_any_element()
            } else {
                div().into_any_element()
            })
    }
}
