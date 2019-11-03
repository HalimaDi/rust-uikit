
use objc::{
    runtime::{
        Sel,
        Object,
    },
};
use objc_foundation::INSObject;
use objc_id::ShareId;

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

pub trait IUIControl : INSObject {
    extern fn new_event(this: &Object, _cmd: Sel) {
        debug!("New Event handler!: {:?}, {:?}?", this, *this.class());
        //unsafe { *this.get_ivar("_number") }
    }
    fn add_event<T: IUIControl>(&self, target: ShareId<T>, event_type: UIControlEvents) {
        unsafe {
            let _: () = msg_send![
                self,
                addTarget:target
                action:sel!(new_event)
                forControlEvents: event_type
            ];
        }
    }

    fn remove_event<T: IUIControl>(&self, target: ShareId<T>, event_type: UIControlEvents) {
        unsafe {
            let _: () = msg_send![
                self,
                actionsForTarget:target
                forControlEvent: event_type
            ];
        }
    }

    fn send_action<T: IUIControl>(&self, event_type: UIControlEvents) {
        unsafe {
            let _: () = msg_send![
                self,
                sendActionsForControlEvents: event_type
            ];
        }
    }
}
