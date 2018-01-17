#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
pub mod macros; /* Handy macros */
mod vpp; /* VPP bindings */

use vpp::*;
use std::ptr;


#[no_mangle]
pub unsafe extern "C" fn rust_test_cli(vm: *mut vlib_main_t, input: *mut unformat_input_t, cmd: *mut vlib_cli_command_t) -> *mut clib_error_t {
  let mut error: *mut clib_error_t = ptr::null_mut();
  let mut val: u32 = 42;

  println!("Hello from a Rust CLI! input: {:?}", *input.clone());

  if unformat (input, cstr_mut!("%u"), &val) == 0 {
      error = _clib_error_return (error, 0, 0, cstr_mut!("in rust plugin code"),
                      cstr_mut!("expecting a number, got `%U`"),
                      format_unformat_error as *mut i8, input);
  } else {
      _clib_error(CLIB_ERROR_WARNING as i32, cstr_mut!("rust-plugin"), line!() as u64, cstr_mut!("got value: %u"), val);
  }

  return error;
}



#[no_mangle]
pub extern "C" fn rust_plugin_init(vm: *mut vlib_main_t) -> *const clib_error_t {

  let mut cli = vlib_cli_command_t {
    path:                      cstr_mut!("rust test"),
    short_help:                cstr_mut!("rust short help"),
    long_help:                 cstr_mut!("rust long help"),
    function:                  Some(rust_test_cli),
    function_arg:              42,
    is_mp_safe:                0,
    sub_commands:              ptr::null_mut(),
    sub_command_index_by_name: ptr::null_mut(),
    sub_command_positions:     ptr::null_mut(),
    sub_rule_index_by_name:    ptr::null_mut(),
    sub_rules:                 ptr::null_mut(),
    next_cli_command:          ptr::null_mut(),
  };

  unsafe {
    vlib_cli_register(vm, &mut cli);
  }

  println!("Hello from a Rust plugin init!");
  return ptr::null();
}

#[no_mangle]
pub extern "C" fn x_rust_plugin_init() {
    println!("I am a x-init function in the rust plugin, I was called by a loader");
}
#[link_section = ".init_array"]
static INIT: extern fn() = x_rust_plugin_init;


/* This is required to get the plugin registered */
#[link_section = ".vlib_plugin_registration"]
#[no_mangle]
pub static mut vlib_plugin_registration: vlib_plugin_registration_t = vlib_plugin_registration_t { 
  default_disabled: 0,
  version: [0; 32], // FIXME: fill in the version
  version_required: [0; 32],
  early_init: cstr!("rust_plugin_init"),
  description: cstr!("A simple Rust plugin"),
}; 

