extern crate core_graphics;
extern crate objc_foundation;
extern crate objc_id;
#[macro_use] extern crate objc;
extern crate uikit;
extern crate uikit_impl;
extern crate color_backtrace;
extern crate pretty_env_logger;
#[macro_use] extern crate log;


use core_graphics::geometry::{CGPoint, CGRect, CGSize};
use objc_foundation::INSObject;
use objc_id::ShareId;
use objc::{
    Message,
    runtime::{
        Class,
        Sel,
        Object,NO,
    },
    declare::ClassDecl,
};
use uikit::{
    IUILabel,
    IUIView,
    IUIViewController,
    IUIControl,
    IUISwitch,
    UIColor,
    UIImage,
    UIImageView,
    UILabel,
    UIViewController,
    UIProgressView,
    UISwitch,
    UIButton,
};
use uikit_impl::ApplicationDelegate;

const LOGO: &'static [u8] = include_bytes!("../rust_logo.png");

struct ExampleAppDelegate;
impl IUIControl for ExampleAppDelegate {
    extern fn new_event(this: &Object, cmd: Sel) {
        debug!("EXAMPLE APP DELEGATE Event handler!: {:?}, {:?}, {:?}",
               this,
               *this.class(),
               cmd,
               );
    }
}

impl INSObject for ExampleAppDelegate {
    fn class() -> &'static Class {
        match Class::get("ExampleAppDelegate") {
            Some(cls) => cls,
            None => {
                let superclass = class!(NSObject);
                let mut decl = ClassDecl::new("ExampleAppDelegate", superclass).unwrap();
                unsafe {
                    decl.add_method(sel!(new_event), Self::new_event as extern  fn(&Object, Sel));
                }
                decl.register()
            },
        }
    }
}
unsafe impl Message for ExampleAppDelegate { }

fn debug_init() {
    color_backtrace::install_with_settings(
        color_backtrace::Settings::new().verbosity(color_backtrace::Verbosity::Full)
    );
    pretty_env_logger::init();
}

impl ApplicationDelegate for ExampleAppDelegate {
    fn root_view_controller(&self) -> ShareId<UIViewController> {
        debug_init();
        debug!("STARTING THE APP!");

        let green = UIColor::from_rgba(0., 1., 0., 1.).share();
        let blue =  UIColor::from_rgba(0., 0., 1., 1.).share();
        let red =   UIColor::from_rgba(1., 0., 0., 1.).share();

        let image = UIImage::with_bytes(LOGO).share();

        let root_vc: ShareId<UIViewController> = UIViewController::new().share();
        root_vc.view().set_background_color(green.clone());

        let image_view: ShareId<UIImageView> = UIImageView::with_image(image).share();
        image_view.set_background_color(blue.clone());
        image_view.set_frame(
            CGRect {
                origin: CGPoint { x: 10., y: 10. },
                size: CGSize {
                    width: 50.,
                    height: 50.,
                },
            }
        );
        root_vc.view().add_subview(image_view);

        let label : ShareId<UILabel> = UILabel::new().share();
        label.set_text(&String::from("FOOBAR"));
        label.set_text_color(red.clone());
        //label.set_enable(NO);
        label.set_frame(
            CGRect {
                origin: CGPoint { x: 10., y: 100. },
                size: CGSize {
                    width: 100.,
                    height: 50.,
                },
            }
        );
        root_vc.view().add_subview(label);

        let switch : ShareId<UISwitch> = UISwitch::new().share();
        switch.set_frame(
            CGRect {
                origin: CGPoint { x: 10., y: 200. },
                size: CGSize {
                    width: 100.,
                    height: 50.,
                },
            }
        );
        switch.set_on_tint_color(red.clone());
        switch.set_thumb_tint_color(red);
        switch.add_event(switch.clone(), uikit::UIControlEvents::ValueChanged);
        root_vc.view().add_subview(switch);


        let button : ShareId<UIButton> = UIButton::with_type(uikit::UIButtonType::UIButtonTypePlain).share();
        button.set_background_color(blue);
        button.set_frame(
            CGRect {
                origin: CGPoint { x: 10., y: 200. },
                size: CGSize {
                    width: 100.,
                    height: 50.,
                },
            }
        );



        let progress_view : ShareId<UIProgressView> = UIProgressView::new().share();
        progress_view.set_frame(
            CGRect {
                origin: CGPoint { x: 10., y: 300. },
                size: CGSize {
                    width: 100.,
                    height: 200.,
                },
            }
        );
        progress_view.set_progress(0.3);
        root_vc.view().add_subview(progress_view);



        root_vc
    }

    fn did_finish_launching(&self) -> bool {
        true
    }
}

#[no_mangle]
pub extern "C" fn run_app() {
    uikit_impl::application_main(ExampleAppDelegate);
}
