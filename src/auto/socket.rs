// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use atk_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use Component;
use Object;

glib_wrapper! {
    pub struct Socket(Object<atk_sys::AtkSocket, atk_sys::AtkSocketClass, SocketClass>) @extends Object, @implements Component;

    match fn {
        get_type => || atk_sys::atk_socket_get_type(),
    }
}

impl Socket {
    pub fn new() -> Socket {
        assert_initialized_main_thread!();
        unsafe { Object::from_glib_full(atk_sys::atk_socket_new()).unsafe_cast() }
    }
}

impl Default for Socket {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SOCKET: Option<&Socket> = None;

pub trait AtkSocketExt: 'static {
    fn embed(&self, plug_id: &str);

    fn is_occupied(&self) -> bool;
}

impl<O: IsA<Socket>> AtkSocketExt for O {
    fn embed(&self, plug_id: &str) {
        unsafe {
            atk_sys::atk_socket_embed(self.as_ref().to_glib_none().0, plug_id.to_glib_none().0);
        }
    }

    fn is_occupied(&self) -> bool {
        unsafe {
            from_glib(atk_sys::atk_socket_is_occupied(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for Socket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Socket")
    }
}
