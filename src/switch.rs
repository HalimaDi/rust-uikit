use core_graphics::geometry::CGRect;
use objc::Message;
use objc::runtime::{
    Class,
    Sel,
};
use objc_id::{Id};
use objc_foundation::INSObject;

use {NoSyncSend, IUIView};

impl IUIView for UISwitch {
    fn set_frame(&self, frame: CGRect) {
        assert_main_thread!();
        unsafe {
            msg_send![self, setFrame:frame]
        }
    }
}

#[repr(C)]
pub enum UIControlEvents {
    TouchDown              = 1 <<  0,      // on all touch downs
    TouchDownRepeat        = 1 <<  1,      // on multiple touchdowns (tap count > 1)
    TouchDragInside        = 1 <<  2,
    TouchDragOutside       = 1 <<  3,
    TouchDragEnter         = 1 <<  4,
    TouchDragExit          = 1 <<  5,
    TouchUpInside          = 1 <<  6,
    TouchUpOutside         = 1 <<  7,
    TouchCancel            = 1 <<  8,

    ValueChanged           = 1 << 12,     // sliders, etc.
    PrimaryActionTriggered = 1 << 13,     // semantic action: for buttons, etc.

    EditingDidBegin        = 1 << 16,     // UITextField
    EditingChanged         = 1 << 17,
    EditingDidEnd          = 1 << 18,
    EditingDidEndOnExit    = 1 << 19,     // 'return key' ending editing

    AllTouchEvents         = 0x00000FFF,  // for touch events
    AllEditingEvents       = 0x000F0000,  // for UITextField
    ApplicationReserved    = 0x0F000000,  // range available for application use
    SystemReserved         = 0xF0000000,  // range reserved for internal framework use
    AllEvents              = 0xFFFFFFFF
}

pub struct UISwitch {
    _marker: NoSyncSend,

}

unsafe impl Message for UISwitch { }

impl UISwitch {
    pub fn with_frame(frame: CGRect) -> Id<Self> {
        let cls = Self::class();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, initWithFrame:frame];
            let _ : () = msg_send![
                obj, addTarget:obj action:Sel::register("on_change") forControlEvents: UIControlEvents::ValueChanged
            ];
            Id::from_retained_ptr(obj)
        }
    }
    pub fn on_change(&self) {
        println!("SOMETHING CHANGED");
    }
}

impl INSObject for UISwitch {
    fn class() -> &'static Class {
        Class::get("UISwitch").unwrap()
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

