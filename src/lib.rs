//! Adaptation of https://sqlite.org/loadext.html#programming_loadable_extensions
//!
//! # build
//! ```sh
//! cargo build --example loadable_extension --features "loadable_extension functions trace"
//! ```
//!
//! # test
//! ```sh
//! sqlite> .log on
//! sqlite> .load target/debug/examples/libloadable_extension.so
//! (28) Rusqlite extension initialized
//! sqlite> SELECT rusqlite_test_function();
//! Rusqlite extension loaded correctly!
//! ```
use rusqlite::ffi;
use std::os::raw::{c_char, c_int};

use rusqlite::functions::FunctionFlags;
use rusqlite::types::{ToSqlOutput, Value};
use rusqlite::{Connection, Result};

/// Entry point for SQLite to load the extension.
/// See <https://sqlite.org/c3ref/load_extension.html> on this function's name and usage.
/// # Safety
/// This function is called by SQLite and must be safe to call.
#[expect(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub unsafe extern "C" fn sqlite3_extension_init(
    db: *mut ffi::sqlite3,
    pz_err_msg: *mut *mut c_char,
    p_api: *mut ffi::sqlite3_api_routines,
) -> c_int {
    Connection::extension_init2(db, pz_err_msg, p_api, extension_init)
}

fn extension_init(db: Connection) -> Result<bool> {
    db.create_scalar_function(
        "decancer_string",
        1, 
        FunctionFlags::SQLITE_DETERMINISTIC, 
        |ctx | {
            let arg = ctx.get::<String>(0)?;
            let cured = decancer::cure!(&arg).unwrap();

            Ok(ToSqlOutput::Owned(Value::Text(
                format!("{}", &cured)
            )))
        }
    )?;

    rusqlite::trace::log(ffi::SQLITE_WARNING, "rs-decancer-sqlite extension initialized");
    Ok(false)
}