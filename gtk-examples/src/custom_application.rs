use std::ptr;
use std::mem;
use std::ops::Deref;
use std::sync::{Once, ONCE_INIT};

use glib;
use glib::prelude::*;
use gtk;
use gtk::prelude::*;
use glib::translate::*;
use gobject_ffi;
use glib_ffi;
use gtk_ffi;
use gio;
use gio_ffi;

use gobject_subclass::object::*;

use gio_subclass::application::{Application as GApplication,
                                ApplicationImpl as GApplicationImpl};

use gtk_subclass::application::*;


mod imp {
    use super::*;

    pub struct CustomApplication {}

    static PROPERTIES: [Property; 0] = [];

    impl CustomApplication {
        pub fn get_type() -> glib::Type {
            static ONCE: Once = ONCE_INIT;
            static mut TYPE: glib::Type = glib::Type::Invalid;

            ONCE.call_once(|| {
                let static_instance = CustomApplicationStatic;
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
    }

    impl ObjectImpl<Object> for CustomApplication {}

    impl ObjectImpl<Application> for CustomApplication {
        fn set_property(&self, _obj: &glib::Object, id: u32, value: &glib::Value) {
            let prop = &PROPERTIES[id as usize];

            match *prop {
                _ => unimplemented!(),
            }
        }

        fn get_property(&self, _obj: &glib::Object, id: u32) -> Result<glib::Value, ()> {
            let prop = &PROPERTIES[id as usize];

            match *prop {
                _ => unimplemented!(),
            }
        }
    }
    impl GApplicationImpl<Application> for CustomApplication {}
    impl ApplicationImpl<Application> for CustomApplication {}

    pub struct CustomApplicationStatic;

    impl ImplTypeStatic<Application> for CustomApplicationStatic {
        fn get_name(&self) -> &str {
            "CustomApplication"
        }

        fn new(&self, application: &Application) -> Box<ApplicationImpl<Application>> {
            CustomApplication::init(application)
        }

        fn class_init(&self, klass: &mut ApplicationClass) {
            CustomApplication::class_init(klass);
        }
    }
}


glib_wrapper! {
    pub struct CustomApplication(Object<imp::CustomApplication>):
        [Application => InstanceStruct<Application>,
         GApplication => InstanceStruct<GApplication>,
         gtk::Application => gtk_ffi::GtkApplication,
         gio::Application => gio_ffi::GApplication];

    match fn {
        get_type => || imp::CustomApplication::get_type().to_glib(),
     }
 }


impl CustomApplication {
    pub fn new<'a, I: Into<Option<&'a str>>>(application_id: I, flags: gio::ApplicationFlags) -> Result<CustomApplication, glib::BoolError> {
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
impl Deref for CustomApplication {
    type Target = imp::CustomApplication;

    fn deref(&self) -> &Self::Target {
        unsafe {

            let base: Application = from_glib_borrow(self.to_glib_none().0);
            let imp = base.get_impl();
            let imp = imp.downcast_ref::<imp::CustomApplication>().unwrap();
            // Cast to a raw pointer to get us an appropriate lifetime: the compiler
            // can't know that the lifetime of base is the same as the one of self
            &*(imp as *const imp::CustomApplication)
        }
    }
}
