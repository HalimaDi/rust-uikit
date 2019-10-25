use core_graphics::geometry::CGRect;
use objc::Message;
use objc::{
    runtime::{
        Class,
        Sel,
        Object,
    },
    declare::{
        ClassDecl,
    },
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
            let _ : () = msg_send![ obj, addTarget:obj action:sel!(number) forControlEvents: UIControlEvents::ValueChanged ];
            Id::from_retained_ptr(obj)
        }
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

pub struct RustSwitch {
}

impl RustSwitch {
    pub fn register() {
    }
    extern fn my_number_get(this: &Object, cmd: Sel) -> u32 {
        debug!("Event handler added for: {:?}, {:?}?, {:?}", this, *this.class(), cmd.name());
        unsafe { *this.get_ivar("_number") }
    }

    pub fn add_event(&self, event_type: UIControlEvents) {
        unsafe {
            let _ : () = msg_send![self, addTarget:self action:sel!(number) forControlEvents: event_type ];
        }
    }
    pub fn with_frame(frame: CGRect) -> Id<Self> {
        let cls = Self::class();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, initWithFrame:frame];
            Id::from_retained_ptr(obj)
        }
    }

}
impl INSObject for RustSwitch {
    fn class() -> &'static Class {

        if let Some(cls) = Class::get("RustSwitch") {
            cls
        } else {
            let superclass = class!(UISwitch);
            let mut decl = ClassDecl::new("RustSwitch", superclass).unwrap();
            decl.add_ivar::<u32>("_number");
            unsafe {
                decl.add_method(sel!(number),
                Self::my_number_get as extern fn(&Object, Sel) -> u32);
            }
            decl.register()
        }
    }
}
unsafe impl Message for RustSwitch { }
impl IUIView for RustSwitch {
    fn set_frame(&self, frame: CGRect) {
        assert_main_thread!();
        unsafe {
            msg_send![self, setFrame:frame]
        }
    }
}
