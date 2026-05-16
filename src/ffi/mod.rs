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
        request_json: *const c_char,
        out_frame: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_from_rows_json(
        rows_json: *const c_char,
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
    pub fn td_dataframe_any_column_json(
        frame: *mut c_void,
        column_name: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn td_dataframe_column_slice_json(
        frame: *mut c_void,
        column_name: *const c_char,
        start: usize,
        end: usize,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn td_dataframe_column_mask_json(
        frame: *mut c_void,
        column_name: *const c_char,
        mask_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn td_dataframe_row_json(
        frame: *mut c_void,
        index: usize,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn td_dataframe_rows_json(frame: *mut c_void, error_out: *mut *mut c_char) -> *mut c_char;
    pub fn td_dataframe_any_rows_json(
        frame: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn td_dataframe_append_row_json(
        frame: *mut c_void,
        row_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_insert_row_json(
        frame: *mut c_void,
        index: usize,
        row_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_replace_row_json(
        frame: *mut c_void,
        index: usize,
        row_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_summary(
        frame: *mut c_void,
        out_frame: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_summary_columns(
        frame: *mut c_void,
        columns_json: *const c_char,
        out_frame: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_summary_indices(
        frame: *mut c_void,
        indices_json: *const c_char,
        out_frame: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_filter_json(
        frame: *mut c_void,
        filter_json: *const c_char,
        out_frame: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_sort_json(
        frame: *mut c_void,
        sort_json: *const c_char,
        out_frame: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_slice_rows(
        frame: *mut c_void,
        start: usize,
        end: usize,
        out_frame: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_prefix_rows(
        frame: *mut c_void,
        len: usize,
        out_frame: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_suffix_rows(
        frame: *mut c_void,
        len: usize,
        out_frame: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_select_columns_json(
        frame: *mut c_void,
        columns_json: *const c_char,
        out_frame: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_join_json(
        frame: *mut c_void,
        other: *mut c_void,
        join_json: *const c_char,
        out_frame: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_group_aggregate_json(
        frame: *mut c_void,
        group_json: *const c_char,
        aggregate_json: *const c_char,
        out_frame: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_group_slice_json(
        frame: *mut c_void,
        group_json: *const c_char,
        keys_json: *const c_char,
        out_frame: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_encode_column_json(
        frame: *mut c_void,
        request_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_decode_column_json(
        frame: *mut c_void,
        request_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_write_csv(
        frame: *mut c_void,
        path: *const c_char,
        options_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn td_dataframe_csv_string(
        frame: *mut c_void,
        options_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
}
