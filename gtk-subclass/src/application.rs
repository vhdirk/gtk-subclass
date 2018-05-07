use std::mem;
use std::ptr;

use cairo;
use cairo_ffi;
use gdk_ffi;
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


// pub fn new(
//     application_id: &str,
//     flags: ApplicationFlags
// ) -> Result<Application, BoolError>


pub trait ApplicationImpl<T: ApplicationBase>: ObjectImpl<T> + AnyImpl + 'static {
    // fn render(
    //     &self,
    //     application: &T,
    //     cr: &cairo::Context,
    //     widget: &gtk::Widget,
    //     background_area: &gtk::Rectangle,
    //     cell_area: &gtk::Rectangle,
    //     flags: gtk::ApplicationState,
    // ) {
    //     application.parent_render(cr, widget, background_area, cell_area, flags)
    // }
}

pub trait ApplicationImplExt<T> {}

impl<S: ApplicationImpl<T>, T: ObjectType + glib::IsA<gtk::Application>> ApplicationImplExt<T>
    for S
{
}

any_impl!(ApplicationBase, ApplicationImpl);

pub unsafe trait ApplicationBase: IsA<gtk::Application> + ObjectType {
    // fn parent_render(
    //     &self,
    //     cr: &cairo::Context,
    //     widget: &gtk::Widget,
    //     background_area: &gtk::Rectangle,
    //     cell_area: &gtk::Rectangle,
    //     flags: gtk::ApplicationState,
    // ) {
    //     unsafe {
    //         let klass = self.get_class();
    //         let parent_klass = (*klass).get_parent_class() as *const gtk_ffi::GtkApplicationClass;
    //         (*parent_klass)
    //             .render
    //             .map(|f| {
    //                 f(
    //                     self.to_glib_none().0,
    //                     cr.to_glib_none().0,
    //                     widget.to_glib_none().0,
    //                     background_area.to_glib_none().0,
    //                     cell_area.to_glib_none().0,
    //                     flags.to_glib(),
    //                 )
    //             })
    //             .unwrap_or(())
    //     }
    // }
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
         gio::Application => gio_ffi::GApplication];

    match fn {
        get_type => || get_type::<Application>(),
    }
}

unsafe impl<T: IsA<gtk::Application> + ObjectType> ApplicationBase for T {}

pub type ApplicationClass = ClassStruct<Application>;

// FIXME: Boilerplate
unsafe impl ApplicationClassExt<Application> for ApplicationClass {}
unsafe impl ObjectClassExt<Application> for ApplicationClass {}

#[macro_export]
macro_rules! box_application_impl(
    ($name:ident) => {
        box_object_impl!($name);

        impl<T: ApplicationBase> ApplicationImpl<T> for Box<$name<T>>
        {
            // fn render(&self, application: &T,
            //                  cr: &cairo::Context,
            //                  widget: &gtk::Widget,
            //                  background_area: &gtk::Rectangle,
            //                  cell_area: &gtk::Rectangle,
            //                  flags: gtk::ApplicationState)
            // {
            //     let imp: &$name<T> = self.as_ref();
            //     imp.render(application, cr, widget, background_area, cell_area, flags)
            // }
        }
    };
);

box_application_impl!(ApplicationImpl);

impl ObjectType for Application {
    const NAME: &'static str = "RsApplication";
    type ParentType = gtk::Application;
    type ImplType = Box<ApplicationImpl<Self>>;
    type InstanceStructType = InstanceStruct<Self>;

    fn class_init(token: &ClassInitToken, klass: &mut ApplicationClass) {
        ObjectClassExt::override_vfuncs(klass, token);
        ApplicationClassExt::override_vfuncs(klass, token);
    }

    object_type_fns!();
}

// unsafe extern "C" fn application_render<T: ApplicationBase>(
//     ptr: *mut gtk_ffi::GtkApplication,
//     cr: *mut cairo_ffi::cairo_t,
//     widget: *mut gtk_ffi::GtkWidget,
//     background_area: *const gdk_ffi::GdkRectangle,
//     cell_area: *const gdk_ffi::GdkRectangle,
//     flags: gtk_ffi::GtkApplicationState,
// ) where
//     T::ImplType: ApplicationImpl<T>,
// {
//     callback_guard!();
//     floating_reference_guard!(ptr);
//     let application = &*(ptr as *mut T::InstanceStructType);
//     let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
//     let imp = application.get_impl();
//
//     imp.render(
//         &wrap,
//         &from_glib_borrow(cr),
//         &from_glib_borrow(widget),
//         &from_glib_borrow(background_area),
//         &from_glib_borrow(cell_area),
//         from_glib(flags),
//     )
// }
