#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
pub mod macros; /* Handy macros */
mod vpp; /* VPP bindings */

use vpp::*;
use std::ptr;
use std::mem::size_of;


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
  let vm = test_vlib_main.unwrap();
  let n = vlib_get_node_by_name(vm, ucstr_mut!("l2-input-classify"));
  let edn = vlib_get_node_by_name(vm, ucstr_mut!("error-drop"));
  let next_index = vlib_node_add_next_with_slot(vm, test_node_index as u64, (*edn).index as u64, 0);
  let next_index = vlib_node_add_next_with_slot(vm, (*n).index as u64, test_node_index as u64, !0u64 );
  println!("Next index is: {}", next_index);

  return error;
}

#[no_mangle]
pub unsafe extern "C" fn test_node_fn (vm: *mut vlib_main_t, node: *mut vlib_node_runtime_t, frame: *mut vlib_frame_t) -> u64
{
  return 0;
}

#[no_mangle]
unsafe extern "C" fn test_format_trace_fn(s: *mut u8, args: *mut va_list) -> *mut u8
{
  return ptr::null_mut();
}

#[no_mangle]
unsafe extern "C" fn test_unformat_trace_fn(input: *mut unformat_input_t, args: *mut va_list) -> uword
{
  return 0;
}

pub static mut test_node_index: u32 = 0;
pub static mut test_vlib_main: Option<*mut vlib_main_t> = None;

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

  let node_index = unsafe {
    vlib_cli_register(vm, &mut cli);
    /*
     * the reason for this fun with flags is here:
     * https://www.reddit.com/r/rust/comments/75pnn2/ffi_and_variable_size_data_structure/
     */
    let mut foo = &mut *(malloc(size_of::<_vlib_node_registration>()) as *mut _vlib_node_registration);
    foo.function = Some(test_node_fn);
    foo.name = cstr_mut!("a-0-test-rust-node");
    foo.type_ = vlib_node_type_t_VLIB_NODE_TYPE_INTERNAL;
    foo.vector_size = 4;
    foo.format_trace = Some(test_format_trace_fn);
    foo.unformat_trace = Some(test_unformat_trace_fn);
    vlib_register_node(vm, foo);
    foo.index
  };

  println!("Hello from a Rust plugin init, node index is {:?}!", node_index);
  unsafe {
    test_node_index = node_index;
    test_vlib_main = Some(vm);
  }
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

