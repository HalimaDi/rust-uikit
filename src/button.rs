use core_graphics::geometry::CGRect;
use objc::Message;
use objc::runtime::Class;
use objc_id::{Id};
use objc_foundation::INSObject;

use {NoSyncSend, IUIView};

impl IUIView for UIButton {
    fn set_frame(&self, frame: CGRect) {
        assert_main_thread!();
        unsafe {
            msg_send![self, setFrame:frame]
        }
    }
}
#[repr(C)]
pub enum UIButtonType {
    UIButtonTypeCustom = 0,
    UIButtonTypeSystem,
    UIButtonTypeDetailDisclosure,
    UIButtonTypeInfoLight,
    UIButtonTypeInfoDark,
    UIButtonTypeContactAdd,
    UIButtonTypePlain,
    UIButtonTypeRoundedRect,
}

pub struct UIButton {
    _marker: NoSyncSend,
}

unsafe impl Message for UIButton { }

impl UIButton {
    pub fn with_type(button_type: UIButtonType) -> Id<Self> {
        let cls = Self::class();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, buttonWithType:button_type];
            //let obj: Result<*mut Self, MessageError> = (*obj).send_message(sel!(buttonWithType:), (button_type as i64));
            Id::from_retained_ptr(obj)
        }
    }
}

impl INSObject for UIButton {
    fn class() -> &'static Class {
        Class::get("UIButton").unwrap()
    }

    // Redefine new to only allow constructing on the main thread
    fn new() -> Id<Self> {
        assert_main_thread!();
        let cls = Self::class();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, init];
            Id::from_retained_ptr(obj)
        }
    }
}
