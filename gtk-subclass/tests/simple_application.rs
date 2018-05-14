//! # Basic test
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and
//! position, how to add a `button` to this `window` and how to connect signals with
//! actions.

use std::ptr;
use std::mem;
use std::ops::Deref;
use std::sync::{Once, ONCE_INIT};

extern crate gdk;
extern crate gio;
extern crate gtk;
#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gtk_sys as gtk_ffi;
extern crate gio_sys as gio_ffi;

extern crate cairo;
extern crate pango;

#[macro_use]
extern crate gtk_test;

#[macro_use]
extern crate gobject_subclass;

#[macro_use]
extern crate gio_subclass;
extern crate gtk_subclass;

use glib::translate::*;
use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

use gobject_subclass::object::*;

use gio_subclass::application::{Application as GApplication,
                                ApplicationImpl as GApplicationImpl,
                                ApplicationBase as GApplicationBase};

use gtk_subclass::application::*;



// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

mod imp {
    use super::*;

    pub struct SimpleApplication;

    static PROPERTIES: [Property; 0] = [];

    impl SimpleApplication {
        pub fn get_type() -> glib::Type {
            static ONCE: Once = ONCE_INIT;
            static mut TYPE: glib::Type = glib::Type::Invalid;

            ONCE.call_once(|| {
                let static_instance = SimpleApplicationStatic;
                let t = register_type(static_instance);
                unsafe {
                    TYPE = t;
                }
            });

            unsafe { TYPE }
        }

        fn class_init(klass: &mut ApplicationClass) {
            klass.install_properties(&PROPERTIES);
        }

        fn init(_application: &Application) -> Box<ApplicationImpl<Application>> {
            let imp = Self {};
            Box::new(imp)
        }

        fn build_ui(&self, application: &Application)
        {
            let window = gtk::ApplicationWindow::new(application);

            window.set_title("First GTK+ Program");
            window.set_border_width(10);
            window.set_position(gtk::WindowPosition::Center);
            window.set_default_size(350, 70);

            window.connect_delete_event(clone!(window => move |_, _| {
                window.destroy();
                gtk::Inhibit(false)
            }));

            let button = gtk::Button::new_with_label("Click me!");

            window.add(&button);

            window.show_all();
        }
    }

    impl GApplicationImpl<Application> for SimpleApplication
    {
        fn startup(&self, application: &Application){
            application.parent_startup();

            self.build_ui(application);
        }
    }

    impl ApplicationImpl<Application> for SimpleApplication {}

    pub struct SimpleApplicationStatic;

    impl ImplTypeStatic<Application> for SimpleApplicationStatic
    {
        fn get_name(&self) -> &str {
            "SimpleApplication"
        }

        fn new(&self, application: &Application) -> Box<ApplicationImpl<Application>> {
            SimpleApplication::init(application)
        }

        fn class_init(&self, klass: &mut ApplicationClass) {
            SimpleApplication::class_init(klass);
        }
    }
}


glib_wrapper! {
    pub struct SimpleApplication(Object<imp::SimpleApplication>):
        [Application => InstanceStruct<Application>,
         GApplication => InstanceStruct<GApplication>,
         gtk::Application => gtk_ffi::GtkApplication,
         gio::Application => gio_ffi::GApplication,
         gio::ActionGroup => gio_ffi::GActionGroup,
         gio::ActionMap => gio_ffi::GActionMap];

    match fn {
        get_type => || imp::SimpleApplication::get_type().to_glib(),
     }
 }


impl SimpleApplication {
    pub fn new<'a, I: Into<Option<&'a str>>>(application_id: I, flags: gio::ApplicationFlags) -> Result<SimpleApplication, glib::BoolError> {
        use glib::object::Downcast;
        try!(gtk::init());

        unsafe {
            match glib::Object::new(Self::static_type(), &[("application_id", &application_id.into()),
                                                           ("flags", &flags)]){
                Ok(obj) => Ok(obj.downcast_unchecked()),
                Err(_) => Err(glib::BoolError("Failed to create application"))
            }
        }
    }
}

// TODO: This one should probably get a macro
impl Deref for SimpleApplication {
    type Target = imp::SimpleApplication;

    fn deref(&self) -> &Self::Target {
        unsafe {

            let base: Application = from_glib_borrow(self.to_glib_none().0);
            let imp = base.get_impl();
            let imp = imp.downcast_ref::<imp::SimpleApplication>().unwrap();
            // Cast to a raw pointer to get us an appropriate lifetime: the compiler
            // can't know that the lifetime of base is the same as the one of self
            &*(imp as *const imp::SimpleApplication)
        }
    }
}




#[test]
fn test_create() {
    let application = SimpleApplication::new("com.github.basic",
                                            gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_activate(|_| {});

    application.run(&["--local".to_string()]);
}
