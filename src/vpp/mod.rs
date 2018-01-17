#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables, unused_mut)]
#![allow(unused)]

// use std;
use std::fmt::{Debug, Formatter, Error};

#[repr(C)]
pub struct vnet_sw_interface_t;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// unsafe impl std::marker::Send for  vlib_plugin_registration_t { }



