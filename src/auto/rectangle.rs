// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use atk_sys;
use gobject_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Rectangle(Boxed<atk_sys::AtkRectangle>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(atk_sys::atk_rectangle_get_type(), ptr as *mut _) as *mut atk_sys::AtkRectangle,
        free => |ptr| gobject_sys::g_boxed_free(atk_sys::atk_rectangle_get_type(), ptr as *mut _),
        get_type => || atk_sys::atk_rectangle_get_type(),
    }
}
