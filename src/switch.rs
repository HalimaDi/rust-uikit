use objc::{
    Message,
    declare::ClassDecl,
    runtime::{Class, Object,Sel,BOOL},
};
use objc_id::ShareId;
use objc_foundation::INSObject;

use crate::{IUIControl, IUIView, UIColor,};

pub trait IUISwitch : INSObject {
    fn set_on(&self, on: BOOL, animated: BOOL) {
        unsafe {
            let _: () = msg_send![
                self,
                setOn:on
                animated:animated
            ];
        }
    }
    fn set_thumb_tint_color(&self, color: ShareId<UIColor>) {
        unsafe {
            let _: () = msg_send![
                self,
                setThumbTintColor: color
            ];
        }
    }
    fn set_on_tint_color(&self, color: ShareId<UIColor>) {
        unsafe {
            let _: () = msg_send![
                self,
                setOnTintColor: color
            ];
        }
    }
}

pub struct UISwitch {}
impl IUIControl for UISwitch {}
impl IUISwitch for UISwitch {}

impl INSObject for UISwitch {
    fn class() -> &'static Class {
        let cls_name = "RustUISwitch";
        match Class::get(cls_name) {
            Some(cls) => cls,
            None => {
                let superclass = class!(UISwitch);
                let mut decl = ClassDecl::new(cls_name, superclass).unwrap();
                unsafe {
                    decl.add_method(
                        sel!(new_event),
                        Self::new_event as extern "C" fn(&Object, Sel),
                    );
                }
                decl.register()
            }
        }
    }
}
unsafe impl Message for UISwitch {}
impl IUIView for UISwitch {}
