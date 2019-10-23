extern crate core_graphics;
extern crate objc_foundation;
extern crate objc_id;
extern crate uikit;
extern crate uikit_impl;
extern crate color_backtrace;
extern crate pretty_env_logger;
#[macro_use] extern crate log;


use core_graphics::geometry::{CGPoint, CGRect, CGSize};
use objc_foundation::INSObject;
use objc_id::ShareId;
use uikit::{
    IUILabel,
    IUIView,
    IUIViewController,
    UIColor,
    UIImage,
    UIImageView,
    UILabel,
    UIViewController,
    UIProgressView,
    UISwitch,
    RustSwitch,
};
use uikit_impl::ApplicationDelegate;

const LOGO: &'static [u8] = include_bytes!("../rust_logo.png");

struct ExampleAppDelegate;

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
        let _red =   UIColor::from_rgba(1., 0., 0., 1.).share();

        let image = UIImage::with_bytes(LOGO).share();

        let root_vc: ShareId<UIViewController> = UIViewController::new().share();
        root_vc.view().set_background_color(green);

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
        RustSwitch::register();
        let switch : ShareId<RustSwitch> = RustSwitch::with_frame(
            CGRect {
                origin: CGPoint { x: 10., y: 200. },
                size: CGSize {
                    width: 100.,
                    height: 50.,
                },
            }
        ).share();
        switch.add_event(uikit::UIControlEvents::AllEvents);
        root_vc.view().add_subview(switch);

        /*
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
        */



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
