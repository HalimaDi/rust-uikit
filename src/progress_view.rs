
use core_graphics::geometry::CGRect;
use objc::Message;
use objc::runtime::Class;
use objc_id::{Id};
use objc_foundation::INSObject;

use {NoSyncSend, IUIView};

impl IUIView for UIProgressView {
    fn set_frame(&self, frame: CGRect) {
        assert_main_thread!();
        unsafe {
            msg_send![self, setFrame:frame]
        }
    }
}

pub struct UIProgressView {
    _marker: NoSyncSend,
}

impl UIProgressView {
    pub fn set_progress(&self, progress: f32) {
        assert_main_thread!();
        unsafe {
            msg_send![self, setProgress:progress animated:true]
        }
    }
}

unsafe impl Message for UIProgressView { }

impl INSObject for UIProgressView {
    fn class() -> &'static Class {
        Class::get("UIProgressView").unwrap()
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
