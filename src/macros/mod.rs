
/* Handy macros to convert strings to C strings */

macro_rules! cstr {
  ($s:expr) => (
    concat!($s, "\0") as *const str as *const [i8] as *const i8
  );
}

macro_rules! cstr_mut {
  ($s:expr) => (
    concat!($s, "\0") as *const str as *mut [i8] as *mut i8
  );
}


