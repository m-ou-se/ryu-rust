use std::ffi::CStr;
use std::mem::uninitialized;
use std::ops::Deref;
use std::os::raw::c_char;
use std::str::from_utf8_unchecked;
use std::slice;

extern {
	fn d2s_buffered(_: f64, _: *mut c_char);
	fn f2s_buffered(_: f32, _: *mut c_char);
}

pub struct F64String {
	buf: [c_char; 25],
}

pub fn d2s(value: f64) -> F64String {
	unsafe {
		let mut s = F64String{ buf: uninitialized() };
		d2s_buffered(value, s.buf.as_mut_ptr());
		let len = CStr::from_ptr(s.buf.as_ptr()).to_bytes().len();
		s.buf[24] = 24 - len as i8;
		s
	}
}

impl Deref for F64String {
	type Target = str;
	fn deref(&self) -> &str {
		unsafe {
			let ptr = self.buf.as_ptr();
			let len = (24 - self.buf[24]) as usize;
			from_utf8_unchecked(slice::from_raw_parts(ptr as *const u8, len))
		}
	}
}

impl std::fmt::Display for F64String {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", &**self)
	}
}

pub struct F32String {
	buf: [c_char; 16],
}

pub fn f2s(value: f32) -> F32String {
	unsafe {
		let mut s = F32String{ buf: uninitialized() };
		f2s_buffered(value, s.buf.as_mut_ptr());
		let len = CStr::from_ptr(s.buf.as_ptr()).to_bytes().len();
		s.buf[15] = 15 - len as i8;
		s
	}
}

impl Deref for F32String {
	type Target = str;
	fn deref(&self) -> &str {
		unsafe {
			let ptr = self.buf.as_ptr();
			let len = (15 - self.buf[15]) as usize;
			from_utf8_unchecked(slice::from_raw_parts(ptr as *const u8, len))
		}
	}
}

impl std::fmt::Display for F32String {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", &**self)
	}
}

#[test]
fn d2s_test() {
	assert_eq!(&*d2s(0.3), "3E-1");
	assert_eq!(&*d2s(1.3), "1.3E0");
	assert_eq!(format!("{} + {} = {}", d2s(0.1), d2s(0.2), d2s(0.3)), "1E-1 + 2E-1 = 3E-1");
}

#[test]
fn f2s_test() {
	assert_eq!(&*f2s(0.3), "3E-1");
	assert_eq!(&*f2s(1.3), "1.3E0");
	assert_eq!(format!("{} + {} = {}", f2s(0.1), f2s(0.2), f2s(0.3)), "1E-1 + 2E-1 = 3E-1");
}
