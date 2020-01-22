extern crate core_graphics;
#[macro_use]
extern crate objc;
extern crate objc_id;
extern crate objc_foundation;
extern crate block;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

macro_rules! assert_main_thread {
    () => (
        assert!($crate::is_main_thread())
    );
}

mod app;
mod color;
mod image;
mod image_view;
mod view;
mod view_controller;
mod label;
mod progress_view;
mod button;
mod switch;
mod control;
mod text_field;

pub use app::{application_main, IUIApplicationDelegate};
pub use color::UIColor;
pub use image::UIImage;
pub use image_view::UIImageView;
pub use view::{IUIView, UIView};
pub use view_controller::{IUIViewController, UIViewController};
pub use label::{
    IUILabel,
    UILabel
};
pub use progress_view::{
    UIProgressView,
};
pub use button::{
    UIButton,
    UIButtonType,
};
pub use switch::{
    UISwitch,
    IUISwitch,
};
pub use control::{
    IUIControl,
    UIControlEvents,
};
pub use text_field::{
    IUITextField,
    UITextField,
};

#[link(name = "UIKit", kind = "framework")]
extern { }

fn is_main_thread() -> bool {
    use objc::runtime::{Class, BOOL, NO};

    let cls = Class::get("NSThread").unwrap();
    let result: BOOL = unsafe { msg_send![cls, isMainThread] };
    result != NO
}

struct NoSyncSend {
    _marker: ::std::marker::PhantomData<*mut ::std::os::raw::c_void>,
}
