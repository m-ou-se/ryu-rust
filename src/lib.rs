use std::cmp::min;
use std::mem::uninitialized;
use std::ops::Deref;
use std::os::raw::{c_char, c_int};
use std::slice;
use std::str::from_utf8_unchecked;
use std::fmt;

extern {
	fn d2s_buffered_n(_: f64, _: *mut c_char) -> c_int;
	fn f2s_buffered_n(_: f32, _: *mut c_char) -> c_int;
}

pub struct F64String {
	buf: [c_char; 24],
}

pub fn d2s(value: f64) -> F64String {
	unsafe {
		let mut s = F64String{ buf: uninitialized() };
		let len = d2s_buffered_n(value, s.buf.as_mut_ptr());
		if len < 24 { s.buf[23] = len as i8; }
		s
	}
}

impl Deref for F64String {
	type Target = str;
	fn deref(&self) -> &str {
		unsafe {
			let ptr = self.buf.as_ptr();
			let len = min(self.buf[23], 24) as usize;
			from_utf8_unchecked(slice::from_raw_parts(ptr as *const u8, len))
		}
	}
}

impl fmt::Display for F64String {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", &**self)
	}
}

impl fmt::Debug for F64String {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", &**self)
	}
}

pub struct F32String {
	buf: [c_char; 15],
}

pub fn f2s(value: f32) -> F32String {
	unsafe {
		let mut s = F32String{ buf: uninitialized() };
		let len = f2s_buffered_n(value, s.buf.as_mut_ptr());
		if len < 15 { s.buf[14] = len as i8; }
		s
	}
}

impl Deref for F32String {
	type Target = str;
	fn deref(&self) -> &str {
		unsafe {
			let ptr = self.buf.as_ptr();
			let len = min(self.buf[14], 15) as usize;
			from_utf8_unchecked(slice::from_raw_parts(ptr as *const u8, len))
		}
	}
}

impl fmt::Display for F32String {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", &**self)
	}
}

impl fmt::Debug for F32String {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", &**self)
	}
}

#[test]
fn d2s_test() {
	assert_eq!(&*d2s(0.3), "3E-1");
	assert_eq!(&*d2s(1.3), "1.3E0");
	assert_eq!(&*d2s(-1.1234567890123456e-300), "-1.1234567890123456E-300"); // Maximum length output.
	assert_eq!(format!("{} + {} = {}", d2s(0.1), d2s(0.2), d2s(0.3)), "1E-1 + 2E-1 = 3E-1");
	assert_eq!(format!("{:?} + {:?} = {:?}", d2s(0.1), d2s(0.2), d2s(0.3)), "1E-1 + 2E-1 = 3E-1");
}

#[test]
fn f2s_test() {
	assert_eq!(&*f2s(0.3), "3E-1");
	assert_eq!(&*f2s(1.3), "1.3E0");
	assert_eq!(&*f2s(-1.00014165e-36), "-1.00014165E-36"); // Maximum length output.
	assert_eq!(format!("{} + {} = {}", f2s(0.1), f2s(0.2), f2s(0.3)), "1E-1 + 2E-1 = 3E-1");
	assert_eq!(format!("{:?} + {:?} = {:?}", f2s(0.1), f2s(0.2), f2s(0.3)), "1E-1 + 2E-1 = 3E-1");
}
