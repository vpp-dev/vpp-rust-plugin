use std::ptr;

#[repr(C)]
pub struct clib_error_t;
#[repr(C)]
pub struct unformat_input_t;

#[repr(C)]
pub struct vlib_init_function_list_elt {
  next_init_function: *mut vlib_init_function_list_elt,
  f: extern "C" fn(*mut vlib_main_t) -> *const clib_error_t,
  /* FIXME: Without the padding there are strange crashes. Need to understand better what is going on */
  padding: [u8; 64],
}

#[repr(C)]
pub struct vlib_main_t;

#[repr(C)]
pub struct vlib_cli_command_t {
  path: *const u8,
  short_help: *const u8,
  long_help: *const u8,
  function: extern "C" fn(*mut vlib_main_t, *mut unformat_input_t, *mut vlib_cli_command_t) -> *const clib_error_t,
  /* There are unused fields, but nonetheless some padding is needed for the same reasons as in function_list_elt structure */
  padding: [u8; 64],
} 

extern "C" {
  fn vlib_cli_register(vm: *mut vlib_main_t, cli: *mut vlib_cli_command_t) -> *const clib_error_t;
}

#[no_mangle]
pub extern "C" fn my_cli(vm: *mut vlib_main_t, input: *mut unformat_input_t, cmd: *mut vlib_cli_command_t) -> *const clib_error_t {
  println!("Hello from a Rust CLI!");
  return ptr::null();
}

pub extern "C" fn aytest() {
  println!("aytest")
}

#[no_mangle]
pub extern "C" fn rust_plugin_init(vm: *mut vlib_main_t) -> *const clib_error_t {
  let path = b"rust test\0"; // \0\0\0\0\0\0\0"; // this was needed to prevent crashing while the padding was not in place.
  let short_help = b"short help\0"; // \0\0\0\0\0\0\0";
  let long_help = b"rust long help\0";
  let foo = b"asd\0".as_ptr();

  let mut cli = vlib_cli_command_t  {
    path: b"rust foo\0".as_ptr(),
    short_help: short_help.as_ptr(),
    long_help: long_help.as_ptr(),
    function: my_cli,
    padding: [0; 64],
  };

  unsafe {
    vlib_cli_register(vm, &mut cli);
    cli.path = path.as_ptr();
    vlib_cli_register(vm, &mut cli);
  }

  println!("Hello from a Rust plugin init!");
  return ptr::null();
}

#[no_mangle]
pub extern "C" fn vlib_plugin_register(vm: *mut vlib_main_t, h: *const i32, from_early_init: i32) -> *const clib_error_t {

  rust_plugin_init(vm);
  return ptr::null();
}

/*

This contains the magic incantations to make the code executed at library load time.
Does happen even before vlib_plugin_register, so probably is not useful.

#[no_mangle]
fn rust_plugin_init() {
    println!("I was called by .init");
}

#[link_section = ".init_array"]
static INIT: fn() = rust_plugin_init;
*/


