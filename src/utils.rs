use core::ffi::CStr;

// to compare a const char* to a &str
pub unsafe fn strcmp(s1: *const i8, s2: &str) -> bool {
	match CStr::from_ptr(s1).to_str() {
		Ok(s1c) => {
			return s1c == s2;
		},
		// just return false cause this shouldn't really happen
		Err(_) => { return false; }
	};
}
