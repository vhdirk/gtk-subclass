use gio;
use gio_ffi;
use glib;
use glib::translate::*;
use glib::IsA;
use glib_ffi;
use gobject_ffi;
use libc;
use std::mem;
use std::ptr;

use gobject_subclass::anyimpl::*;
use gobject_subclass::object::*;

pub trait ApplicationImpl<T: ApplicationBase>: ObjectImpl<T> + AnyImpl + 'static {
    fn startup(&self, application: &T) {
        application.parent_startup();
    }

    fn activate(&self, application: &T) {
        application.parent_activate();
    }

    fn open(&self, application: &T, files: &Vec<gio::File>, hint: &String) {
        application.parent_open(files, hint)
    }

    fn command_line(&self, application: &T, cmd_line: &gio::ApplicationCommandLine) -> i32 {
        application.parent_command_line(cmd_line)
    }

    fn local_command_line(&self, application: &T, arguments: &Vec<String>) -> Option<i32> {
        application.parent_local_command_line(arguments)
    }
}

pub trait ApplicationImplExt<T> {}

impl<S: ApplicationImpl<T>, T: ObjectType + glib::IsA<gio::Application>> ApplicationImplExt<T>
    for S
{
}

any_impl!(ApplicationBase, ApplicationImpl);

pub unsafe trait ApplicationBase: IsA<gio::Application> + ObjectType {
    fn parent_startup(&self) {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
                .startup
                .map(|f| f(self.to_glib_none().0))
                .unwrap_or(())
        }
    }

    fn parent_activate(&self) {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
                .activate
                .map(|f| f(self.to_glib_none().0))
                .unwrap_or(())
        }
    }

    fn parent_open(&self, files: &Vec<gio::File>, hint: &String) {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
                .open
                .map(|f| {
                    let n_files = files.len() as i32;
                    f(
                        self.to_glib_none().0,
                        files.to_glib_none().0,
                        n_files,
                        hint.to_glib_none().0,
                    )
                })
                .unwrap_or(())
        }
    }

    fn parent_command_line(&self, cmd_line: &gio::ApplicationCommandLine) -> i32 {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
                .command_line
                .map(|f| f(self.to_glib_none().0, cmd_line.to_glib_none().0))
                .unwrap_or(0)
        }
    }

    fn parent_local_command_line(&self, arguments: &Vec<String>) -> Option<i32> {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            let mut exit_status = 0;
            (*parent_klass)
                .local_command_line
                .map(|f| {
                    f(
                        self.to_glib_none().0,
                        arguments.to_glib_none().0, //TODO: conversion
                        &mut exit_status,
                    )
                })
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
            klass.startup = Some(application_startup::<T>);
            klass.activate = Some(application_activate::<T>);
            klass.open = Some(application_open::<T>);
            klass.command_line = Some(application_command_line::<T>);
            klass.local_command_line = Some(application_local_command_line::<T>);
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
            fn startup(&self, application: &T){
                let imp: &$name<T> = self.as_ref();
                imp.startup(application)
            }

            fn activate(&self, application: &T){
                let imp: &$name<T> = self.as_ref();
                imp.activate(application)
            }

            fn open(&self, application: &T, files: &Vec<gio::File>, hint: &String){
                let imp: &$name<T> = self.as_ref();
                imp.open(application, files, hint)
            }

            fn command_line(&self, application: &T, cmd_line: &gio::ApplicationCommandLine) -> i32{
                let imp: &$name<T> = self.as_ref();
                imp.command_line(application, cmd_line)
            }

            fn local_command_line(&self, application: &T, arguments: &Vec<String>) -> Option<i32>{
                let imp: &$name<T> = self.as_ref();
                imp.local_command_line(application, arguments)
            }
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

unsafe extern "C" fn application_startup<T: ApplicationBase>(ptr: *mut gio_ffi::GApplication)
where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    imp.startup(&wrap)
}

unsafe extern "C" fn application_activate<T: ApplicationBase>(ptr: *mut gio_ffi::GApplication)
where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    imp.activate(&wrap)
}

unsafe extern "C" fn application_open<T: ApplicationBase>(
    ptr: *mut gio_ffi::GApplication,
    files: *mut *mut gio_ffi::GFile,
    num_files: libc::c_int,
    hint: *const libc::c_char,
) where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    imp.open(
        &wrap,
        &FromGlibContainer::from_glib_none_num(files, num_files as usize),
        &from_glib_none(hint),
    )
}

unsafe extern "C" fn application_command_line<T: ApplicationBase>(
    ptr: *mut gio_ffi::GApplication,
    cmd_line: *mut gio_ffi::GApplicationCommandLine,
) -> libc::c_int
where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    imp.command_line(&wrap, &from_glib_borrow(cmd_line))
}

unsafe extern "C" fn application_local_command_line<T: ApplicationBase>(
    ptr: *mut gio_ffi::GApplication,
    arguments: *mut *mut *mut libc::c_char,
    exit_status: *mut libc::c_int,
) -> glib_ffi::gboolean
where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    //TODO: conversion
    match imp.local_command_line(&wrap, &from_glib_borrow(arguments)) {
        Some(status) => {
            *exit_status = status;
            1
        }
        None => 0,
    }
}

//  pub before_emit: Option<unsafe extern "C" fn(_: *mut GApplication, _: *mut GVariant)>,
//  pub after_emit: Option<unsafe extern "C" fn(_: *mut GApplication, _: *mut GVariant)>,
//  pub add_platform_data: Option<unsafe extern "C" fn(_: *mut GApplication, _: *mut GVariantBuilder)>,
//  pub quit_mainloop: Option<unsafe extern "C" fn(_: *mut GApplication)>,
//  pub run_mainloop: Option<unsafe extern "C" fn(_: *mut GApplication)>,
//  pub shutdown: Option<unsafe extern "C" fn(_: *mut GApplication)>,
//  pub dbus_register: Option<unsafe extern "C" fn(_: *mut GApplication, _: *mut GDBusConnection, _: *const c_char, _: *mut *mut GError) -> gboolean>,
//  pub dbus_unregister: Option<unsafe extern "C" fn(_: *mut GApplication, _: *mut GDBusConnection, _: *const c_char)>,
//  pub handle_local_options: Option<unsafe extern "C" fn(_: *mut GApplication, _: *mut GVariantDict) -> c_int>,
//  pub padding: [gpointer; 8],
