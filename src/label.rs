use objc::Message;
use objc::runtime::Class;
use objc_id::{Id};
use objc_foundation::{INSObject, NSString, INSString};

use crate::{NoSyncSend, IUIView};

pub trait IUILabel : INSObject {
    fn set_text(&self, text: &str) {//-> ShareId<UILabel> {
        assert_main_thread!();
        let text = NSString::from_str(text);
        unsafe {
            let _ : *mut Self=  msg_send![self, setText:text];
        }
    }
}

pub struct UILabel {
    _marker: NoSyncSend,
}

unsafe impl Message for UILabel { }

impl INSObject for UILabel {
    fn class() -> &'static Class {
        Class::get("UILabel").unwrap()
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

impl IUILabel for UILabel { }
impl IUIView for UILabel { }