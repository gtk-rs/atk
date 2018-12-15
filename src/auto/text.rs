// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use CoordType;
use TextBoundary;
use TextClipType;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use TextGranularity;
use TextRange;
use TextRectangle;
use ffi;
use glib::GString;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct Text(Object<ffi::AtkText, ffi::AtkTextIface>);

    match fn {
        get_type => || ffi::atk_text_get_type(),
    }
}

pub trait TextExt: 'static {
    fn add_selection(&self, start_offset: i32, end_offset: i32) -> bool;

    fn get_bounded_ranges(&self, rect: &mut TextRectangle, coord_type: CoordType, x_clip_type: TextClipType, y_clip_type: TextClipType) -> Vec<TextRange>;

    fn get_caret_offset(&self) -> i32;

    fn get_character_at_offset(&self, offset: i32) -> char;

    fn get_character_count(&self) -> i32;

    fn get_character_extents(&self, offset: i32, coords: CoordType) -> (i32, i32, i32, i32);

    //fn get_default_attributes(&self) -> /*Ignored*/Option<AttributeSet>;

    fn get_n_selections(&self) -> i32;

    fn get_offset_at_point(&self, x: i32, y: i32, coords: CoordType) -> i32;

    fn get_range_extents(&self, start_offset: i32, end_offset: i32, coord_type: CoordType) -> TextRectangle;

    //fn get_run_attributes(&self, offset: i32) -> (/*Ignored*/AttributeSet, i32, i32);

    fn get_selection(&self, selection_num: i32) -> (GString, i32, i32);

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_string_at_offset(&self, offset: i32, granularity: TextGranularity) -> (Option<GString>, i32, i32);

    fn get_text(&self, start_offset: i32, end_offset: i32) -> Option<GString>;

    #[cfg_attr(feature = "v2_9_3", deprecated)]
    fn get_text_after_offset(&self, offset: i32, boundary_type: TextBoundary) -> (GString, i32, i32);

    fn get_text_at_offset(&self, offset: i32, boundary_type: TextBoundary) -> (GString, i32, i32);

    #[cfg_attr(feature = "v2_9_3", deprecated)]
    fn get_text_before_offset(&self, offset: i32, boundary_type: TextBoundary) -> (GString, i32, i32);

    fn remove_selection(&self, selection_num: i32) -> bool;

    fn set_caret_offset(&self, offset: i32) -> bool;

    fn set_selection(&self, selection_num: i32, start_offset: i32, end_offset: i32) -> bool;

    fn connect_text_attributes_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_text_caret_moved<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v2_9_4", deprecated)]
    fn connect_text_changed<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_text_insert<F: Fn(&Self, i32, i32, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_text_remove<F: Fn(&Self, i32, i32, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_text_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Text>> TextExt for O {
    fn add_selection(&self, start_offset: i32, end_offset: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_text_add_selection(self.to_glib_none().0, start_offset, end_offset))
        }
    }

    fn get_bounded_ranges(&self, rect: &mut TextRectangle, coord_type: CoordType, x_clip_type: TextClipType, y_clip_type: TextClipType) -> Vec<TextRange> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::atk_text_get_bounded_ranges(self.to_glib_none().0, rect.to_glib_none_mut().0, coord_type.to_glib(), x_clip_type.to_glib(), y_clip_type.to_glib()))
        }
    }

    fn get_caret_offset(&self) -> i32 {
        unsafe {
            ffi::atk_text_get_caret_offset(self.to_glib_none().0)
        }
    }

    fn get_character_at_offset(&self, offset: i32) -> char {
        unsafe {
            from_glib(ffi::atk_text_get_character_at_offset(self.to_glib_none().0, offset))
        }
    }

    fn get_character_count(&self) -> i32 {
        unsafe {
            ffi::atk_text_get_character_count(self.to_glib_none().0)
        }
    }

    fn get_character_extents(&self, offset: i32, coords: CoordType) -> (i32, i32, i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::atk_text_get_character_extents(self.to_glib_none().0, offset, &mut x, &mut y, &mut width, &mut height, coords.to_glib());
            (x, y, width, height)
        }
    }

    //fn get_default_attributes(&self) -> /*Ignored*/Option<AttributeSet> {
    //    unsafe { TODO: call ffi::atk_text_get_default_attributes() }
    //}

    fn get_n_selections(&self) -> i32 {
        unsafe {
            ffi::atk_text_get_n_selections(self.to_glib_none().0)
        }
    }

    fn get_offset_at_point(&self, x: i32, y: i32, coords: CoordType) -> i32 {
        unsafe {
            ffi::atk_text_get_offset_at_point(self.to_glib_none().0, x, y, coords.to_glib())
        }
    }

    fn get_range_extents(&self, start_offset: i32, end_offset: i32, coord_type: CoordType) -> TextRectangle {
        unsafe {
            let mut rect = TextRectangle::uninitialized();
            ffi::atk_text_get_range_extents(self.to_glib_none().0, start_offset, end_offset, coord_type.to_glib(), rect.to_glib_none_mut().0);
            rect
        }
    }

    //fn get_run_attributes(&self, offset: i32) -> (/*Ignored*/AttributeSet, i32, i32) {
    //    unsafe { TODO: call ffi::atk_text_get_run_attributes() }
    //}

    fn get_selection(&self, selection_num: i32) -> (GString, i32, i32) {
        unsafe {
            let mut start_offset = mem::uninitialized();
            let mut end_offset = mem::uninitialized();
            let ret = from_glib_full(ffi::atk_text_get_selection(self.to_glib_none().0, selection_num, &mut start_offset, &mut end_offset));
            (ret, start_offset, end_offset)
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_string_at_offset(&self, offset: i32, granularity: TextGranularity) -> (Option<GString>, i32, i32) {
        unsafe {
            let mut start_offset = mem::uninitialized();
            let mut end_offset = mem::uninitialized();
            let ret = from_glib_full(ffi::atk_text_get_string_at_offset(self.to_glib_none().0, offset, granularity.to_glib(), &mut start_offset, &mut end_offset));
            (ret, start_offset, end_offset)
        }
    }

    fn get_text(&self, start_offset: i32, end_offset: i32) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::atk_text_get_text(self.to_glib_none().0, start_offset, end_offset))
        }
    }

    fn get_text_after_offset(&self, offset: i32, boundary_type: TextBoundary) -> (GString, i32, i32) {
        unsafe {
            let mut start_offset = mem::uninitialized();
            let mut end_offset = mem::uninitialized();
            let ret = from_glib_full(ffi::atk_text_get_text_after_offset(self.to_glib_none().0, offset, boundary_type.to_glib(), &mut start_offset, &mut end_offset));
            (ret, start_offset, end_offset)
        }
    }

    fn get_text_at_offset(&self, offset: i32, boundary_type: TextBoundary) -> (GString, i32, i32) {
        unsafe {
            let mut start_offset = mem::uninitialized();
            let mut end_offset = mem::uninitialized();
            let ret = from_glib_full(ffi::atk_text_get_text_at_offset(self.to_glib_none().0, offset, boundary_type.to_glib(), &mut start_offset, &mut end_offset));
            (ret, start_offset, end_offset)
        }
    }

    fn get_text_before_offset(&self, offset: i32, boundary_type: TextBoundary) -> (GString, i32, i32) {
        unsafe {
            let mut start_offset = mem::uninitialized();
            let mut end_offset = mem::uninitialized();
            let ret = from_glib_full(ffi::atk_text_get_text_before_offset(self.to_glib_none().0, offset, boundary_type.to_glib(), &mut start_offset, &mut end_offset));
            (ret, start_offset, end_offset)
        }
    }

    fn remove_selection(&self, selection_num: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_text_remove_selection(self.to_glib_none().0, selection_num))
        }
    }

    fn set_caret_offset(&self, offset: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_text_set_caret_offset(self.to_glib_none().0, offset))
        }
    }

    fn set_selection(&self, selection_num: i32, start_offset: i32, end_offset: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_text_set_selection(self.to_glib_none().0, selection_num, start_offset, end_offset))
        }
    }

    fn connect_text_attributes_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"text-attributes-changed\0".as_ptr() as *const _,
                transmute(text_attributes_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_text_caret_moved<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"text-caret-moved\0".as_ptr() as *const _,
                transmute(text_caret_moved_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_text_changed<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32, i32) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"text-changed\0".as_ptr() as *const _,
                transmute(text_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_text_insert<F: Fn(&Self, i32, i32, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32, i32, &str) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"text-insert\0".as_ptr() as *const _,
                transmute(text_insert_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_text_remove<F: Fn(&Self, i32, i32, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32, i32, &str) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"text-remove\0".as_ptr() as *const _,
                transmute(text_remove_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_text_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"text-selection-changed\0".as_ptr() as *const _,
                transmute(text_selection_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn text_attributes_changed_trampoline<P>(this: *mut ffi::AtkText, f: glib_ffi::gpointer)
where P: IsA<Text> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Text::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn text_caret_moved_trampoline<P>(this: *mut ffi::AtkText, arg1: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<Text> {
    let f: &&(Fn(&P, i32) + 'static) = transmute(f);
    f(&Text::from_glib_borrow(this).downcast_unchecked(), arg1)
}

unsafe extern "C" fn text_changed_trampoline<P>(this: *mut ffi::AtkText, arg1: libc::c_int, arg2: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<Text> {
    let f: &&(Fn(&P, i32, i32) + 'static) = transmute(f);
    f(&Text::from_glib_borrow(this).downcast_unchecked(), arg1, arg2)
}

unsafe extern "C" fn text_insert_trampoline<P>(this: *mut ffi::AtkText, arg1: libc::c_int, arg2: libc::c_int, arg3: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<Text> {
    let f: &&(Fn(&P, i32, i32, &str) + 'static) = transmute(f);
    f(&Text::from_glib_borrow(this).downcast_unchecked(), arg1, arg2, &GString::from_glib_borrow(arg3))
}

unsafe extern "C" fn text_remove_trampoline<P>(this: *mut ffi::AtkText, arg1: libc::c_int, arg2: libc::c_int, arg3: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<Text> {
    let f: &&(Fn(&P, i32, i32, &str) + 'static) = transmute(f);
    f(&Text::from_glib_borrow(this).downcast_unchecked(), arg1, arg2, &GString::from_glib_borrow(arg3))
}

unsafe extern "C" fn text_selection_changed_trampoline<P>(this: *mut ffi::AtkText, f: glib_ffi::gpointer)
where P: IsA<Text> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Text::from_glib_borrow(this).downcast_unchecked())
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Text")
    }
}
