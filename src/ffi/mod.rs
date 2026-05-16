use core::ffi::{c_char, c_void};

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const FRAMEWORK_ERROR: i32 = -2;
}

unsafe extern "C" {
    pub fn td_object_release(ptr: *mut c_void);

    pub fn td_dataframe_new(out_frame: *mut *mut c_void, error_out: *mut *mut c_char) -> i32;
    pub fn td_dataframe_from_csv(
        path: *const c_char,
        options_json: *const c_char,
        out_frame: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_shape(frame: *mut c_void, out_rows: *mut usize, out_columns: *mut usize);
    pub fn td_dataframe_column_names_json(
        frame: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn td_dataframe_append_column(
        frame: *mut c_void,
        column_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_rename_column(
        frame: *mut c_void,
        column_name: *const c_char,
        new_name: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_column_json(
        frame: *mut c_void,
        column_name: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn td_dataframe_rows_json(frame: *mut c_void, error_out: *mut *mut c_char) -> *mut c_char;
    pub fn td_dataframe_summary(
        frame: *mut c_void,
        out_frame: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_joined(
        frame: *mut c_void,
        other: *mut c_void,
        column_name: *const c_char,
        join_kind: i32,
        out_frame: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_write_csv(
        frame: *mut c_void,
        path: *const c_char,
        options_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
}
