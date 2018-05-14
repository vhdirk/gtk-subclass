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
    ObjectImpl<T> + GApplicationImpl<T> + AnyImpl + 'static
{
    fn window_added(&self, application: &T, window: &gtk::Window) {
        application.parent_window_added(window)
    }

    fn window_removed(&self, application: &T, window: &gtk::Window) {
        application.parent_window_removed(window)
    }
}

pub trait ApplicationImplExt<T> {}


any_impl!(ApplicationBase, ApplicationImpl);

pub unsafe trait ApplicationBase:
    IsA<gtk::Application> + IsA<gio::Application> + ObjectType {

    fn parent_window_added(&self, window: &gtk::Window) {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gtk_ffi::GtkApplicationClass;
            (*parent_klass)
                .window_added
                .map(|f| f(self.to_glib_none().0, window.to_glib_none().0))
                .unwrap_or(())
        }
    }

    fn parent_window_removed(&self, window: &gtk::Window) {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gtk_ffi::GtkApplicationClass;
            (*parent_klass)
                .window_removed
                .map(|f| f(self.to_glib_none().0, window.to_glib_none().0))
                .unwrap_or(())
        }
    }
}

pub unsafe trait ApplicationClassExt<T: ApplicationBase>
where
    T::ImplType: ApplicationImpl<T>,
{
    fn override_vfuncs(&mut self, _: &ClassInitToken) {
        unsafe {
            let klass = &mut *(self as *const Self as *mut gtk_ffi::GtkApplicationClass);
            klass.window_added = Some(application_window_added::<T>);
            klass.window_removed = Some(application_window_removed::<T>);
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
            fn window_added(&self, application: &T, window: &gtk::Window){
                let imp: &$name<T> = self.as_ref();
                imp.window_added(application, window)
            }

            fn window_removed(&self, application: &T, window: &gtk::Window){
                let imp: &$name<T> = self.as_ref();
                imp.window_removed(application, window)
            }
        }
    };
);

box_gtk_application_impl!(ApplicationImpl);

impl ObjectType for Application {
    const NAME: &'static str = "RsGtkApplication";
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


unsafe extern "C" fn application_window_added<T: ApplicationBase>(
    ptr: *mut gtk_ffi::GtkApplication,
    window: *mut gtk_ffi::GtkWindow)
where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    imp.window_added(&wrap, &from_glib_borrow(window))
}

unsafe extern "C" fn application_window_removed<T: ApplicationBase>(
    ptr: *mut gtk_ffi::GtkApplication,
    window: *mut gtk_ffi::GtkWindow)
where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    imp.window_removed(&wrap, &from_glib_borrow(window))
}
