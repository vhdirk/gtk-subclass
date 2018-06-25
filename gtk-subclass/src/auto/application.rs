// This file was generated by gir (https://github.com/gtk-rs/gir @ 573f58b+)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8+)
// DO NOT EDIT

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib_ffi::{gboolean, gconstpointer, gpointer, GType};

pub use cell_renderer::*;

use gio;
use gio_ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use gtk_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

use gobject_subclass::anyimpl::*;
use gobject_subclass::object::*;
use gio_subclass::application as gio_application;

use gio_subclass::application::ArgumentList;

pub trait ApplicationImpl<T: ApplicationBase>: gio_application::ApplicationImpl<T> + ObjectImpl<T> + AnyImpl + 'static {

    fn window_added(&self, application: &T, window: &gtk::Window){
        application.parent_window_added(window)
    }

    fn window_removed(&self, application: &T, window: &gtk::Window){
        application.parent_window_removed(window)
    }

}

pub trait ApplicationImplExt<T> {}
impl<S: ApplicationImpl<T>, T: ObjectType + glib::IsA<gtk::Application> + glib::IsA<gio::Application>> ApplicationImplExt<T> for S {}

any_impl!(ApplicationBase, ApplicationImpl);

pub unsafe trait ApplicationBase: ObjectType + glib::IsA<gtk::Application> + glib::IsA<gio::Application>{

    fn parent_window_added(&self, window: &gtk::Window){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gtk_ffi::GtkApplicationClass;
            (*parent_klass)
            .window_added
            .map(|f|{ f(self.to_glib_none().0,window.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_window_removed(&self, window: &gtk::Window){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gtk_ffi::GtkApplicationClass;
            (*parent_klass)
            .window_removed
            .map(|f|{ f(self.to_glib_none().0,window.to_glib_none().0); })
            .unwrap_or(())
        }
    }

}

pub unsafe trait ApplicationClassExt<T: ApplicationBase>
where
    T::ImplType: ApplicationImpl<T>{

    fn override_vfuncs(&mut self, _: &ClassInitToken){
        unsafe {
            let klass = &mut *(self as *const Self as *mut gtk_ffi::GtkApplicationClass);
            klass.window_added = Some(application_window_added::<T>);
            klass.window_removed = Some(application_window_removed::<T>);
        }
    }

}

glib_wrapper! {

    pub struct Application(Object<InstanceStruct<Application>>):[
         gtk::Application => gtk_ffi::GtkApplication,
         gio::Application => gio_ffi::GApplication,
         gio::ActionGroup => gio_ffi::GActionGroup,
         gio::ActionMap => gio_ffi::GActionMap,]    ;
    match fn { 
         get_type => || get_type::<Application>(),
     }

}

unsafe impl<T: ObjectType + glib::IsA<gtk::Application> + glib::IsA<gio::Application>> ApplicationBase for T {}

pub type ApplicationClass = ClassStruct<Application>;

// FIXME: Boilerplate
unsafe impl ObjectClassExt<Application> for ApplicationClass {}
unsafe impl ApplicationClassExt<Application> for ApplicationClass {}
unsafe impl gio_application::ApplicationClassExt<Application> for ApplicationClass {}

#[macro_export]
macro_rules! box_gtk_application_impl(
    ($name:ident) => {
        box_gio_application_impl!($name);
        impl<T: $crate::application::ApplicationBase> $crate::application::ApplicationImpl<T> for Box<$name<T>>{

            fn window_added(&self, application: &T, window: &gtk::Window){
                let imp: &$name<T> = self.as_ref();
                imp.window_added(application, window)
            }

            fn window_removed(&self, application: &T, window: &gtk::Window){
                let imp: &$name<T> = self.as_ref();
                imp.window_removed(application, window)
            }
        }
    }
);

box_gtk_application_impl!(ApplicationImpl);

impl ObjectType for Application{
    const NAME: &'static str = "RsGtkApplication";
    type ParentType = gtk::Application;
    type ImplType = Box<ApplicationImpl<Self>>;
    type InstanceStructType = InstanceStruct<Self>;
    fn class_init(token: &ClassInitToken, klass: &mut ApplicationClass) {
        ObjectClassExt::override_vfuncs(klass, token);
        ApplicationClassExt::override_vfuncs(klass, token);
        gio_application::ApplicationClassExt::override_vfuncs(klass, token);
    }
    object_type_fns!();
}

unsafe extern "C" fn application_window_added<T: ApplicationBase>
(gptr: *mut gtk_ffi::GtkApplication, window: *mut gtk_ffi::GtkWindow)
where
    T::ImplType: ApplicationImpl<T>
{
    floating_reference_guard!(gptr);
    let application = &*(gptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(gptr as *mut T::InstanceStructType);
    let imp = application.get_impl();
    imp.window_added(&wrap, &from_glib_none(window))
}

unsafe extern "C" fn application_window_removed<T: ApplicationBase>
(gptr: *mut gtk_ffi::GtkApplication, window: *mut gtk_ffi::GtkWindow)
where
    T::ImplType: ApplicationImpl<T>
{
    floating_reference_guard!(gptr);
    let application = &*(gptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(gptr as *mut T::InstanceStructType);
    let imp = application.get_impl();
    imp.window_removed(&wrap, &from_glib_none(window))
}