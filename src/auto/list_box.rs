// This file was generated by gir (8cacc34) from gir-files (11e0e6d)
// DO NOT EDIT

#[cfg(feature = "v3_10")]
use Adjustment;
use Container;
#[cfg(feature = "v3_10")]
use ListBoxRow;
use MovementStep;
#[cfg(feature = "v3_10")]
use SelectionMode;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct ListBox(Object<ffi::GtkListBox>): Container, Widget;

    match fn {
        get_type => || ffi::gtk_list_box_get_type(),
    }
}

impl ListBox {
    #[cfg(feature = "v3_10")]
    pub fn new() -> ListBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_list_box_new()).downcast_unchecked()
        }
    }

    //#[cfg(feature = "v3_16")]
    //pub fn bind_model<T: IsA</*Ignored*/gio::ListModel>>(&self, model: Option<&T>, create_widget_func: /*Unknown conversion*//*Unimplemented*/ListBoxCreateWidgetFunc, user_data: /*Unimplemented*/Fundamental: Pointer, user_data_free_func: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_list_box_bind_model() }
    //}

    #[cfg(feature = "v3_10")]
    pub fn drag_highlight_row(&self, row: &ListBoxRow) {
        unsafe {
            ffi::gtk_list_box_drag_highlight_row(self.to_glib_none().0, row.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn drag_unhighlight_row(&self) {
        unsafe {
            ffi::gtk_list_box_drag_unhighlight_row(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_activate_on_single_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_get_activate_on_single_click(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_adjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_adjustment(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_row_at_index(&self, index_: i32) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_row_at_index(self.to_glib_none().0, index_))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_row_at_y(&self, y: i32) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_row_at_y(self.to_glib_none().0, y))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_selected_row(&self) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_selected_row(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn get_selected_rows(&self) -> Vec<ListBoxRow> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_list_box_get_selected_rows(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_selection_mode(&self) -> SelectionMode {
        unsafe {
            from_glib(ffi::gtk_list_box_get_selection_mode(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn insert<T: IsA<Widget>>(&self, child: &T, position: i32) {
        unsafe {
            ffi::gtk_list_box_insert(self.to_glib_none().0, child.to_glib_none().0, position);
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn invalidate_filter(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_filter(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn invalidate_headers(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_headers(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn invalidate_sort(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_sort(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn prepend<T: IsA<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_list_box_prepend(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn select_all(&self) {
        unsafe {
            ffi::gtk_list_box_select_all(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn select_row(&self, row: Option<&ListBoxRow>) {
        unsafe {
            ffi::gtk_list_box_select_row(self.to_glib_none().0, row.to_glib_none().0);
        }
    }

    //#[cfg(feature = "v3_14")]
    //pub fn selected_foreach(&self, func: /*Unknown conversion*//*Unimplemented*/ListBoxForeachFunc, data: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::gtk_list_box_selected_foreach() }
    //}

    #[cfg(feature = "v3_10")]
    pub fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_list_box_set_activate_on_single_click(self.to_glib_none().0, single.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_adjustment(&self, adjustment: Option<&Adjustment>) {
        unsafe {
            ffi::gtk_list_box_set_adjustment(self.to_glib_none().0, adjustment.to_glib_none().0);
        }
    }

    //#[cfg(feature = "v3_10")]
    //pub fn set_filter_func(&self, filter_func: /*Unknown conversion*//*Unimplemented*/ListBoxFilterFunc, user_data: /*Unimplemented*/Fundamental: Pointer, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_list_box_set_filter_func() }
    //}

    //#[cfg(feature = "v3_10")]
    //pub fn set_header_func(&self, update_header: /*Unknown conversion*//*Unimplemented*/ListBoxUpdateHeaderFunc, user_data: /*Unimplemented*/Fundamental: Pointer, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_list_box_set_header_func() }
    //}

    #[cfg(feature = "v3_10")]
    pub fn set_placeholder<T: IsA<Widget>>(&self, placeholder: Option<&T>) {
        unsafe {
            ffi::gtk_list_box_set_placeholder(self.to_glib_none().0, placeholder.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_selection_mode(&self, mode: SelectionMode) {
        unsafe {
            ffi::gtk_list_box_set_selection_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    //#[cfg(feature = "v3_10")]
    //pub fn set_sort_func(&self, sort_func: /*Unknown conversion*//*Unimplemented*/ListBoxSortFunc, user_data: /*Unimplemented*/Fundamental: Pointer, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_list_box_set_sort_func() }
    //}

    #[cfg(feature = "v3_14")]
    pub fn unselect_all(&self) {
        unsafe {
            ffi::gtk_list_box_unselect_all(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn unselect_row(&self, row: &ListBoxRow) {
        unsafe {
            ffi::gtk_list_box_unselect_row(self.to_glib_none().0, row.to_glib_none().0);
        }
    }

    pub fn connect_activate_cursor_row<F: Fn(&ListBox) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&ListBox) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-cursor-row",
                transmute(activate_cursor_row_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_move_cursor<F: Fn(&ListBox, MovementStep, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&ListBox, MovementStep, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-cursor",
                transmute(move_cursor_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn connect_row_activated<F: Fn(&ListBox, &ListBoxRow) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&ListBox, &ListBoxRow) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "row-activated",
                transmute(row_activated_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn connect_row_selected<F: Fn(&ListBox, &Option<ListBoxRow>) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&ListBox, &Option<ListBoxRow>) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "row-selected",
                transmute(row_selected_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn connect_select_all<F: Fn(&ListBox) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&ListBox) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "select-all",
                transmute(select_all_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn connect_selected_rows_changed<F: Fn(&ListBox) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&ListBox) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "selected-rows-changed",
                transmute(selected_rows_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_toggle_cursor_row<F: Fn(&ListBox) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&ListBox) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "toggle-cursor-row",
                transmute(toggle_cursor_row_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn connect_unselect_all<F: Fn(&ListBox) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&ListBox) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "unselect-all",
                transmute(unselect_all_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_cursor_row_trampoline(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&ListBox) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn move_cursor_trampoline(this: *mut ffi::GtkListBox, object: ffi::GtkMovementStep, p0: libc::c_int, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&ListBox, MovementStep, i32) + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(object), p0)
}

#[cfg(feature = "v3_10")]
unsafe extern "C" fn row_activated_trampoline(this: *mut ffi::GtkListBox, row: *mut ffi::GtkListBoxRow, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&ListBox, &ListBoxRow) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(row))
}

#[cfg(feature = "v3_10")]
unsafe extern "C" fn row_selected_trampoline(this: *mut ffi::GtkListBox, row: *mut ffi::GtkListBoxRow, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&ListBox, &Option<ListBoxRow>) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(row))
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn select_all_trampoline(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&ListBox) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn selected_rows_changed_trampoline(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&ListBox) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn toggle_cursor_row_trampoline(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&ListBox) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn unselect_all_trampoline(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&ListBox) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
