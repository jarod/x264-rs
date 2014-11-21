extern crate libc;

use libc::c_int;
use std::mem;

pub mod ffi;

pub type Param = ffi::x264_param_t;

impl Param {
	fn new() -> Param {
        unsafe{
            mem::zeroed()
        }
    }

	pub fn default() -> Param {
		let mut param = Param::new();
		unsafe {
			ffi::x264_param_default(&mut param);
		}
		param
	}

	pub fn default_preset(preset: &str, tune: &str) -> Param {
		let mut param = Param::new();
		unsafe {
			ffi::x264_param_default_preset(&mut param, 
				preset.to_c_str().as_ptr(), 
				tune.to_c_str().as_ptr());
		}
		param
	}

	pub fn apply_profile(&mut self, profile: &str) {
		unsafe {
			ffi::x264_param_apply_profile(self, profile.to_c_str().as_ptr());
		}
	}
}

#[allow(non_camel_case_types)]
pub enum ColorSpace {
	X264_CSP_I420 = 0x0001,
	X264_CSP_YV12 = 0x0002,
	X264_CSP_NV12 = 0x0003,
	X264_CSP_I422 = 0x0004,
	X264_CSP_YV16 = 0x0005,
	X264_CSP_NV16 = 0x0006,
	X264_CSP_V210 = 0x0007,
	X264_CSP_I444 = 0x0008,
	X264_CSP_YV24 = 0x0009,
	X264_CSP_BGR =  0x000a,
	X264_CSP_BGRA = 0x000b,
	X264_CSP_RGB =  0x000c,
}

pub type Picture = ffi::x264_picture_t;

impl Picture {
	fn new() -> Picture {
        unsafe{
            mem::zeroed()
        }
    }

	pub fn alloc(csp: ColorSpace, width: int, height: int) -> Result<Picture, int> {
     	let mut pic = Picture::new();
     	match unsafe { ffi::x264_picture_alloc(&mut pic, csp as c_int, width as c_int, height as c_int) } {
     		0 => Ok(pic),
     		v => Err(v as int)
     	}
     }

     pub fn clean(&mut self) {
     	unsafe {
            ffi::x264_picture_clean(self);
        }
     }
}

pub type Nal = ffi::x264_nal_t;
impl Nal {
	pub fn new() -> Nal {
		unsafe {
			std::mem::zeroed()
		}
	}
}

pub struct Encoder {
	h: *mut ffi::x264_t
}

impl Encoder {

    pub fn open(param: *mut Param) -> Encoder {
    	let handle = unsafe { ffi::x264_encoder_open(param) };
    	Encoder{ h: handle }
    }

    pub fn encode(&mut self, 
    	pp_nal: *mut *mut Nal, pi_nal: *mut libc::c_int,
    	pic_in: *mut Picture, pic_out: *mut Picture) -> int {
    	unsafe {
    		ffi::x264_encoder_encode(self.h, pp_nal, pi_nal, pic_in, pic_out) as int
    	}
    }

    pub fn close(&mut self) {
    	unsafe { ffi::x264_encoder_close(self.h) }
    }
}