// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use atom::Atom;
use gdk_sys;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;
use crate::{Device, DragAction, DragContext, DragProtocol, Screen, Window};

impl DragContext {
    pub fn drag_get_selection(&self) -> Atom {
        unsafe { from_glib_none(gdk_sys::gdk_drag_get_selection(self.to_glib_none().0) as *mut _) }
    }

    pub fn drag_abort(&self, time_: u32) {
        unsafe { gdk_sys::gdk_drag_abort(self.to_glib_none().0, time_) }
    }

    pub fn drop_reply(&self, accepted: bool, time_: u32) {
        unsafe { gdk_sys::gdk_drop_reply(self.to_glib_none().0, accepted.to_glib(), time_) }
    }

    pub fn drop(&self, time_: u32) {
        unsafe { gdk_sys::gdk_drag_drop(self.to_glib_none().0, time_) }
    }

    pub fn drag_find_window_for_screen(
        &self,
        drag_window: &Window,
        screen: &Screen,
        x_root: i32,
        y_root: i32,
    ) -> (Option<Window>, DragProtocol) {
        unsafe {
            let mut dest_window = ptr::null_mut();
            let mut protocol = gdk_sys::GDK_DRAG_PROTO_NONE;
            gdk_sys::gdk_drag_find_window_for_screen(
                self.to_glib_none().0,
                drag_window.to_glib_none().0,
                screen.to_glib_none().0,
                x_root,
                y_root,
                &mut dest_window,
                &mut protocol,
            );
            (from_glib_full(dest_window), from_glib(protocol))
        }
    }

    pub fn drag_motion(
        &self,
        dest_window: &Window,
        protocol: DragProtocol,
        x_root: i32,
        y_root: i32,
        suggested_action: DragAction,
        possible_actions: DragAction,
        time_: u32,
    ) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_drag_motion(
                self.to_glib_none().0,
                dest_window.to_glib_none().0,
                protocol.to_glib(),
                x_root,
                y_root,
                suggested_action.to_glib(),
                possible_actions.to_glib(),
                time_,
            ))
        }
    }

    pub fn drop_finish(&self, success: bool, time_: u32) {
        unsafe { gdk_sys::gdk_drop_finish(self.to_glib_none().0, success.to_glib(), time_) }
    }

    pub fn drag_status(&self, action: DragAction, time_: u32) {
        unsafe { gdk_sys::gdk_drag_status(self.to_glib_none().0, action.to_glib(), time_) }
    }

    pub fn drag_drop_succeeded(&self) -> bool {
        unsafe { from_glib(gdk_sys::gdk_drag_drop_succeeded(self.to_glib_none().0)) }
    }

    pub fn drag_begin(window: &Window, targets: &[&Atom]) -> Option<DragContext> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gdk_sys::gdk_drag_begin(
                window.to_glib_none().0,
                targets.to_glib_none().0,
            ))
        }
    }

    pub fn drag_begin_for_device<P: IsA<Device>>(
        window: &Window,
        device: &P,
        targets: &[&Atom],
    ) -> Option<DragContext> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gdk_sys::gdk_drag_begin_for_device(
                window.to_glib_none().0,
                device.as_ref().to_glib_none().0,
                targets.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn drag_begin_from_point<P: IsA<Device>>(
        window: &Window,
        device: &P,
        targets: &[&Atom],
        x_root: i32,
        y_root: i32,
    ) -> Option<DragContext> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gdk_sys::gdk_drag_begin_from_point(
                window.to_glib_none().0,
                device.as_ref().to_glib_none().0,
                targets.to_glib_none().0,
                x_root,
                y_root,
            ))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn drag_drop_done(&self, success: bool) {
        skip_assert_initialized!();
        unsafe {
            gdk_sys::gdk_drag_drop_done(self.to_glib_none().0, success.to_glib());
        }
    }
}
