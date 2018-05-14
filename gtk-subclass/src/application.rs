use std::mem;
use std::ptr;

use glib;
use glib::translate::*;
use glib::IsA;
use glib_ffi;
use gobject_ffi;
use gtk;
use gtk_ffi;
use gio;
use gio_ffi;

use gobject_subclass::anyimpl::*;
use gobject_subclass::object::*;

use gio_subclass::application::{ApplicationClassExt as GApplicationClassExt,
                                ApplicationImpl as GApplicationImpl,
                                ArgumentList};


pub trait ApplicationImpl<T: ApplicationBase>:
    ObjectImpl<T> + GApplicationImpl<T> + AnyImpl + 'static {

}

pub trait ApplicationImplExt<T> {}


any_impl!(ApplicationBase, ApplicationImpl);

pub unsafe trait ApplicationBase:
    IsA<gtk::Application> + IsA<gio::Application> + ObjectType {

}

pub unsafe trait ApplicationClassExt<T: ApplicationBase>
where
    T::ImplType: ApplicationImpl<T>,
{
    fn override_vfuncs(&mut self, _: &ClassInitToken) {
        unsafe {
            let klass = &mut *(self as *const Self as *mut gtk_ffi::GtkApplicationClass);
            // klass.render = Some(application_render::<T>);
        }
    }
}

glib_wrapper! {
    pub struct Application(Object<InstanceStruct<Application>>):
    [gtk::Application => gtk_ffi::GtkApplication,
     gio::Application => gio_ffi::GApplication,
     gio::ActionGroup => gio_ffi::GActionGroup,
     gio::ActionMap => gio_ffi::GActionMap];

    match fn {
        get_type => || get_type::<Application>(),
    }
}

unsafe impl<T: IsA<gtk::Application> + IsA<gio::Application> + ObjectType> ApplicationBase for T {}

pub type ApplicationClass = ClassStruct<Application>;

// FIXME: Boilerplate
unsafe impl ApplicationClassExt<Application> for ApplicationClass {}
unsafe impl GApplicationClassExt<Application> for ApplicationClass {}
unsafe impl ObjectClassExt<Application> for ApplicationClass {}

#[macro_export]
macro_rules! box_gtk_application_impl(
    ($name:ident) => {
        box_gapplication_impl!($name);

        impl<T: $crate::application::ApplicationBase> $crate::application::ApplicationImpl<T> for Box<$name<T>>
        {

        }
    };
);

box_gtk_application_impl!(ApplicationImpl);

impl ObjectType for Application {
    const NAME: &'static str = "RsApplication";
    type ParentType = gtk::Application;
    type ImplType = Box<ApplicationImpl<Self>>;
    type InstanceStructType = InstanceStruct<Self>;

    fn class_init(token: &ClassInitToken, klass: &mut ApplicationClass) {
        ObjectClassExt::override_vfuncs(klass, token);
        GApplicationClassExt::override_vfuncs(klass, token);
        ApplicationClassExt::override_vfuncs(klass, token);
    }

    object_type_fns!();
}
