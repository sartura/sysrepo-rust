#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;
use std::ptr::null_mut;

// based on sysrepo sr_get_item_example
fn main() {
    unsafe {
        let app_name = CString::new("rust-app").unwrap().as_ptr();
        let mut conn : *mut sr_conn_ctx_t = null_mut();
        let mut sess : *mut sr_session_ctx_t = null_mut();
        let mut value: *mut sr_val_t = null_mut();
        let item = CString::new("/ietf-interfaces:interfaces/interface[name='eth0']/enabled").unwrap().as_ptr();
        sr_log_stderr(sr_log_level_t_SR_LL_DBG);

        let mut rc = sr_connect(app_name, sr_conn_flag_e_SR_CONN_DEFAULT, &mut conn);
        if rc as u32 != sr_error_e_SR_ERR_OK {
            exit(1);
        }

        rc = sr_session_start(conn, sr_datastore_e_SR_DS_STARTUP, sr_session_flag_e_SR_SESS_DEFAULT, &mut sess);
        if rc as u32 != sr_error_e_SR_ERR_OK {
            sr_disconnect(conn);
            exit(1);
        }

        rc = sr_get_item(sess, item, &mut value);
        if rc as u32 != sr_error_e_SR_ERR_OK {
            sr_session_stop(sess);
            sr_disconnect(conn);
            exit(1);
        }

        sr_print_val(value);
        sr_free_val(value);
    }
}