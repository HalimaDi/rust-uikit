extern crate core_graphics;
extern crate objc_foundation;
extern crate objc_id;
#[macro_use]
extern crate objc;
extern crate pretty_env_logger;
extern crate uikit;
extern crate color_backtrace;
#[macro_use] extern crate log;

use core_graphics::geometry::{CGPoint, CGRect, CGSize};
use objc::{
    declare::ClassDecl,
    runtime::{Class, Object, Sel, NO, },
    Message,
};
use objc_foundation::INSObject;
use objc_id::{
    ShareId,
    Id,
};
use uikit::{
    IUIControl, IUILabel, IUISwitch, IUIView, IUIViewController, UIButton, UIColor, UIImage,
    UIView,
    UIImageView, UILabel, UIProgressView, UISwitch, UIViewController,
};
use uikit_impl::ApplicationDelegate;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{
        WindowBuilder,
        Window
    },
    platform::ios::{
        WindowExtIOS,
        EventLoopExtIOS,
        WindowBuilderExtIOS,
    },
};

const LOGO: &'static [u8] = include_bytes!("../rust_logo.png");
fn debug_init() {
    color_backtrace::install_with_settings(
        color_backtrace::Settings::new().verbosity(color_backtrace::Verbosity::Full),
    );
    pretty_env_logger::init();
    std::env::set_var("RUST_LOG", "DEBUG");
    std::env::set_var("RUST_BACKTRACE", "full");
}

fn run_winit() -> ! {
    let event_loop = EventLoop::new();
    let window : Window = WindowBuilder::new()
        .with_title("UIKit Rust App")
        .build(&event_loop)
        .expect("Failed to build window");

    let root_vc: ShareId<UIViewController> = unsafe {
        Id::from_retained_ptr(&mut *(window.ui_view_controller() as * mut UIViewController))
    }.share();
    root_vc.view().set_background_color(UIColor::from_rgba(0., 1., 0., 1.).share());

    event_loop.run(move |event: Event<()>, _, control_flow: &mut ControlFlow| {
        println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::RedrawRequested(_) => {
            }
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(_logical_size) => {
                    window.request_redraw();
                }
                WindowEvent::Touch(_touch) => {
                    add_views(&root_vc);
                },
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
            _ => (),
        }
    })
}

fn main() -> ! {
    debug_init();
    debug!("STARTING THE APP!");
    run_winit()
    //uikit_impl::application_main(ExampleAppDelegate)
}
fn add_views(root_vc: &ShareId<UIViewController>) {//{{{

        let green = UIColor::from_rgba(0., 1., 0., 1.).share();
        let blue =  UIColor::from_rgba(0., 0., 1., 1.).share();
        let red =   UIColor::from_rgba(1., 0., 0., 1.).share();
        root_vc.view().set_background_color(green.clone());

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
        switch.set_thumb_tint_color(red.clone());
        //switch.add_event(switch.clone(), uikit::UIControlEvents::ValueChanged);
        root_vc.view().add_subview(switch);

        let image = UIImage::with_bytes(LOGO).share();


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


}//}}}

struct ExampleAppDelegate;
impl ApplicationDelegate for ExampleAppDelegate {
    fn root_view_controller(&self) -> ShareId<UIViewController> {
        let root_vc: ShareId<UIViewController> = UIViewController::new().share();
        add_views(&root_vc);
        root_vc
    }
    fn did_finish_launching(&self) -> bool {
        true
    }
}

#[no_mangle]
pub extern "C" fn run_app() {
    main()
}
