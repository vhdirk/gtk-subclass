use std::mem;
use std::ptr;

use glib;
use glib::translate::*;
use glib::IsA;
use glib_ffi;
use gobject_ffi;
use gio;
use gio_ffi;

use gobject_subclass::anyimpl::*;
use gobject_subclass::object::*;


pub trait ApplicationImpl<T: ApplicationBase>: ObjectImpl<T> + AnyImpl + 'static
{
    fn activate(&self, application: &T){
        application.parent_activate();
    }


}

pub trait ApplicationImplExt<T> {}

impl<S: ApplicationImpl<T>, T: ObjectType + glib::IsA<gio::Application>> ApplicationImplExt<T>
    for S
{
}

any_impl!(ApplicationBase, ApplicationImpl);

pub unsafe trait ApplicationBase: IsA<gio::Application> + ObjectType
{
    fn parent_activate(&self) {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass).activate.map(|f| {
                                        f(self.to_glib_none().0)
                                    })
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
            let klass = &mut *(self as *const Self as *mut gio_ffi::GApplicationClass);
            klass.activate = Some(application_activate::<T>);
            // klass.get_application_id = Some(application_get_application_id::<T>);
            // klass.get_dbus_object_path = Some(application_get_dbus_object_path::<T>);
            // klass.get_flags = Some(application_get_flags::<T>);
            // klass.get_inactivity_timeout = Some(application_get_inactivity_timeout::<T>);
            // klass.get_is_busy = Some(application_get_is_busy::<T>);
            // klass.get_is_registered = Some(application_get_is_registered::<T>);
            // klass.get_is_remote = Some(application_get_is_remote::<T>);
            // klass.get_resource_base_path = Some(application_get_resource_base_path::<T>);
            // klass.hold = Some(application_hold::<T>);
            // klass.mark_busy = Some(application_mark_busy::<T>);
            // klass.open = Some(application_open::<T>);
            // klass.quit = Some(application_quit::<T>);
            // fn register<'a, P: Into<Option<&'a Cancellable>>>(
            //     &self,
            //     cancellable: P
            // ) -> Result<(), Error>;
            // fn release(&self);
            // fn send_notification<'a, P: Into<Option<&'a str>>>(
            //     &self,
            //     id: P,
            //     notification: &Notification
            // );
            // fn set_action_group<'a, P: IsA<ActionGroup> + 'a, Q: Into<Option<&'a P>>>(
            //     &self,
            //     action_group: Q
            // );
            // fn set_application_id<'a, P: Into<Option<&'a str>>>(
            //     &self,
            //     application_id: P
            // );
            // fn set_default(&self);
            // fn set_flags(&self, flags: ApplicationFlags);
            // fn set_inactivity_timeout(&self, inactivity_timeout: u32);
            // fn set_resource_base_path<'a, P: Into<Option<&'a str>>>(
            //     &self,
            //     resource_path: P
            // );
            // fn unbind_busy_property<P: IsA<Object>>(&self, object: &P, property: &str);
            // fn unmark_busy(&self);
            // fn withdraw_notification(&self, id: &str);
            // fn get_property_resource_base_path(&self) -> Option<String>;
            // fn set_property_resource_base_path(&self, resource_base_path: Option<&str>);
            // fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
            // fn connect_shutdown<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
            // fn connect_startup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
            // fn connect_property_action_group_notify<F: Fn(&Self) + 'static>(
            //     &self,
            //     f: F
            // ) -> SignalHandlerId;
            // fn connect_property_application_id_notify<F: Fn(&Self) + 'static>(
            //     &self,
            //     f: F
            // ) -> SignalHandlerId;
            // fn connect_property_flags_notify<F: Fn(&Self) + 'static>(
            //     &self,
            //     f: F
            // ) -> SignalHandlerId;
            // fn connect_property_inactivity_timeout_notify<F: Fn(&Self) + 'static>(
            //     &self,
            //     f: F
            // ) -> SignalHandlerId;
            // fn connect_property_is_busy_notify<F: Fn(&Self) + 'static>(
            //     &self,
            //     f: F
            // ) -> SignalHandlerId;
            // fn connect_property_is_registered_notify<F: Fn(&Self) + 'static>(
            //     &self,
            //     f: F
            // ) -> SignalHandlerId;
            // fn connect_property_is_remote_notify<F: Fn(&Self) + 'static>(
            //     &self,
            //     f: F
            // ) -> SignalHandlerId;
            // fn connect_property_resource_base_path_notify<F: Fn(&Self) + 'static>(
            //     &self,
            //     f: F
            // ) -> SignalHandlerId;


        }
    }
}

glib_wrapper! {
    pub struct Application(Object<InstanceStruct<Application>>):
        [gio::Application => gio_ffi::GApplication];

    match fn {
        get_type => || get_type::<Application>(),
    }
}

unsafe impl<T: IsA<gio::Application> + ObjectType> ApplicationBase for T {}

pub type ApplicationClass = ClassStruct<Application>;

// FIXME: Boilerplate
unsafe impl ApplicationClassExt<Application> for ApplicationClass {}
unsafe impl ObjectClassExt<Application> for ApplicationClass {}

#[macro_export]
macro_rules! box_gapplication_impl(
    ($name:ident) => {
        box_object_impl!($name);

        impl<T: $crate::application::ApplicationBase>  $crate::application::ApplicationImpl<T> for Box<$name<T>>
        {

        }
    };
);

box_gapplication_impl!(ApplicationImpl);

impl ObjectType for Application {
    const NAME: &'static str = "RsGApplication";
    type ParentType = gio::Application;
    type ImplType = Box<ApplicationImpl<Self>>;
    type InstanceStructType = InstanceStruct<Self>;

    fn class_init(token: &ClassInitToken, klass: &mut ApplicationClass) {
        ObjectClassExt::override_vfuncs(klass, token);
        ApplicationClassExt::override_vfuncs(klass, token);
    }

    object_type_fns!();
}

unsafe extern "C" fn application_activate<T: ApplicationBase>(
    ptr: *mut gio_ffi::GApplication
) where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    imp.activate(&wrap)
}



// fn bind_busy_property<P: IsA<Object>>(&self, object: &P, property: &str);
// fn get_application_id(&self) -> Option<String>;
// fn get_dbus_object_path(&self) -> Option<String>;
// fn get_flags(&self) -> ApplicationFlags;
// fn get_inactivity_timeout(&self) -> u32;
// fn get_is_busy(&self) -> bool;
// fn get_is_registered(&self) -> bool;
// fn get_is_remote(&self) -> bool;
// fn get_resource_base_path(&self) -> Option<String>;
// fn hold(&self);
// fn mark_busy(&self);
// fn open(&self, files: &[File], hint: &str);
// fn quit(&self);
// fn register<'a, P: Into<Option<&'a Cancellable>>>(
//     &self,
//     cancellable: P
// ) -> Result<(), Error>;
// fn release(&self);
