// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use CoordType;
use Layer;
use Object;
use Rectangle;
#[cfg(any(feature = "v2_30", feature = "dox"))]
use ScrollType;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct Component(Interface<ffi::AtkComponent>);

    match fn {
        get_type => || ffi::atk_component_get_type(),
    }
}

pub const NONE_COMPONENT: Option<&Component> = None;

pub trait ComponentExt: 'static {
    fn contains(&self, x: i32, y: i32, coord_type: CoordType) -> bool;

    fn get_alpha(&self) -> f64;

    fn get_extents(&self, coord_type: CoordType) -> (i32, i32, i32, i32);

    fn get_layer(&self) -> Layer;

    fn get_mdi_zorder(&self) -> i32;

    fn get_position(&self, coord_type: CoordType) -> (i32, i32);

    fn get_size(&self) -> (i32, i32);

    fn grab_focus(&self) -> bool;

    fn ref_accessible_at_point(&self, x: i32, y: i32, coord_type: CoordType) -> Option<Object>;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    fn scroll_to(&self, type_: ScrollType) -> bool;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    fn scroll_to_point(&self, coords: CoordType, x: i32, y: i32) -> bool;

    fn set_extents(&self, x: i32, y: i32, width: i32, height: i32, coord_type: CoordType) -> bool;

    fn set_position(&self, x: i32, y: i32, coord_type: CoordType) -> bool;

    fn set_size(&self, width: i32, height: i32) -> bool;

    fn connect_bounds_changed<F: Fn(&Self, &Rectangle) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Component>> ComponentExt for O {
    fn contains(&self, x: i32, y: i32, coord_type: CoordType) -> bool {
        unsafe {
            from_glib(ffi::atk_component_contains(self.as_ref().to_glib_none().0, x, y, coord_type.to_glib()))
        }
    }

    fn get_alpha(&self) -> f64 {
        unsafe {
            ffi::atk_component_get_alpha(self.as_ref().to_glib_none().0)
        }
    }

    fn get_extents(&self, coord_type: CoordType) -> (i32, i32, i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::atk_component_get_extents(self.as_ref().to_glib_none().0, &mut x, &mut y, &mut width, &mut height, coord_type.to_glib());
            (x, y, width, height)
        }
    }

    fn get_layer(&self) -> Layer {
        unsafe {
            from_glib(ffi::atk_component_get_layer(self.as_ref().to_glib_none().0))
        }
    }

    fn get_mdi_zorder(&self) -> i32 {
        unsafe {
            ffi::atk_component_get_mdi_zorder(self.as_ref().to_glib_none().0)
        }
    }

    fn get_position(&self, coord_type: CoordType) -> (i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            ffi::atk_component_get_position(self.as_ref().to_glib_none().0, &mut x, &mut y, coord_type.to_glib());
            (x, y)
        }
    }

    fn get_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::atk_component_get_size(self.as_ref().to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn grab_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::atk_component_grab_focus(self.as_ref().to_glib_none().0))
        }
    }

    fn ref_accessible_at_point(&self, x: i32, y: i32, coord_type: CoordType) -> Option<Object> {
        unsafe {
            from_glib_full(ffi::atk_component_ref_accessible_at_point(self.as_ref().to_glib_none().0, x, y, coord_type.to_glib()))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    fn scroll_to(&self, type_: ScrollType) -> bool {
        unsafe {
            from_glib(ffi::atk_component_scroll_to(self.as_ref().to_glib_none().0, type_.to_glib()))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    fn scroll_to_point(&self, coords: CoordType, x: i32, y: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_component_scroll_to_point(self.as_ref().to_glib_none().0, coords.to_glib(), x, y))
        }
    }

    fn set_extents(&self, x: i32, y: i32, width: i32, height: i32, coord_type: CoordType) -> bool {
        unsafe {
            from_glib(ffi::atk_component_set_extents(self.as_ref().to_glib_none().0, x, y, width, height, coord_type.to_glib()))
        }
    }

    fn set_position(&self, x: i32, y: i32, coord_type: CoordType) -> bool {
        unsafe {
            from_glib(ffi::atk_component_set_position(self.as_ref().to_glib_none().0, x, y, coord_type.to_glib()))
        }
    }

    fn set_size(&self, width: i32, height: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_component_set_size(self.as_ref().to_glib_none().0, width, height))
        }
    }

    fn connect_bounds_changed<F: Fn(&Self, &Rectangle) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"bounds-changed\0".as_ptr() as *const _,
                Some(transmute(bounds_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn bounds_changed_trampoline<P, F: Fn(&P, &Rectangle) + 'static>(this: *mut ffi::AtkComponent, arg1: *mut ffi::AtkRectangle, f: glib_ffi::gpointer)
where P: IsA<Component> {
    let f: &F = &*(f as *const F);
    f(&Component::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(arg1))
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Component")
    }
}
