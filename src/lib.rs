#![allow(missing_copy_implementations)]

extern crate x264_sys;
#[macro_use]
extern crate lazy_static;

use std::ffi::CString;
use std::mem;
use std::os::raw::{c_char, c_int};
use x264_sys::ffi;

#[allow(non_camel_case_types)]
pub enum ColorSpace {
    X264_CSP_I420 = 0x0001,
    X264_CSP_YV12,
    X264_CSP_NV12,
    X264_CSP_I422,
    X264_CSP_YV16,
    X264_CSP_NV16,
    X264_CSP_V210,
    X264_CSP_I444,
    X264_CSP_YV24,
    X264_CSP_BGR,
    X264_CSP_BGRA,
    X264_CSP_RGB,
}

lazy_static! {
static ref PRESET_NAMES: [CString;10] = [
	CString::new("ultrafast").unwrap(),
	CString::new("superfast").unwrap(),
	CString::new("veryfast").unwrap(),
	CString::new("faster").unwrap(),
	CString::new("fast").unwrap(),
	CString::new("medium").unwrap(),
	CString::new("slow").unwrap(),
	CString::new("slower").unwrap(),
	CString::new("veryslow").unwrap(),
	CString::new("placebo").unwrap(),
];
}
#[derive(Copy, Clone)]
pub enum Preset {
    ULTRAFAST,
    SUPERFAST,
    VERYFAST,
    FASTER,
    FAST,
    MEDIUM,
    SLOW,
    SLOWER,
    VERYSLOW,
    PLACEBO,
}
impl Preset {
    pub fn as_ptr(&self) -> *const c_char {
        PRESET_NAMES[*self as usize].as_ptr()
    }
}

lazy_static! {
static ref TUNE_NAMES: [CString;8] = [
	CString::new("film").unwrap(),
	CString::new("animation").unwrap(),
	CString::new("grain").unwrap(),
	CString::new("stillimage").unwrap(),
	CString::new("psnr").unwrap(),
	CString::new("ssim").unwrap(),
	CString::new("fastdecode").unwrap(),
	CString::new("zerolatency").unwrap(),
];
}
#[derive(Copy, Clone)]
pub enum Tune {
    FILM,
    ANIMATION,
    GRAIN,
    STILLIMAGE,
    PSNR,
    SSIM,
    FASTDECODE,
    ZEROLATENCY,
}
impl Tune {
    pub fn as_ptr(&self) -> *const c_char {
        TUNE_NAMES[*self as usize].as_ptr()
    }
}

lazy_static! {
static ref PROFILE_NAMES: [CString; 6] = [
	CString::new("baseline").unwrap(),
	CString::new("main").unwrap(),
	CString::new("high").unwrap(),
	CString::new("high10").unwrap(),
	CString::new("high422").unwrap(),
	CString::new("high444").unwrap(),
];
}
#[derive(Copy, Clone)]
pub enum Profile {
    BASELINE,
    MAIN,
    HIGH,
    HIGH10,
    HIGH422,
    HIGH444,
}
impl Profile {
    pub fn as_ptr(&self) -> *const c_char {
        PROFILE_NAMES[*self as usize].as_ptr()
    }
}

pub struct Param {
    raw: ffi::x264_param_t,
}
impl Param {
    fn new() -> Param {
        unsafe { mem::zeroed() }
    }

    pub fn default() -> Param {
        let mut param = Param::new();
        unsafe {
            ffi::x264_param_default(&mut param.raw);
        }
        param
    }

    pub fn default_preset(preset: &Preset, tune: &Tune) -> Param {
        let mut param = Param::new();
        unsafe {
            ffi::x264_param_default_preset(&mut param.raw, preset.as_ptr(), tune.as_ptr());
        }
        param
    }

    pub fn apply_profile(&mut self, profile: &Profile) {
        unsafe {
            ffi::x264_param_apply_profile(&mut self.raw, profile.as_ptr());
        }
    }
}


pub struct Picture {
    raw: ffi::x264_picture_t,
}
impl Picture {
    fn new() -> Picture {
        unsafe { mem::zeroed() }
    }

    pub fn alloc(csp: ColorSpace, width: isize, height: isize) -> Result<Picture, isize> {
        let mut pic = Picture::new();
        match unsafe {
            ffi::x264_picture_alloc(&mut pic.raw, csp as c_int, width as c_int, height as c_int)
        } {
            0 => Ok(pic),
            v => Err(v as isize),
        }
    }
}
impl Drop for Picture {
    fn drop(&mut self) {
        unsafe { ffi::x264_picture_clean(&mut self.raw) }
    }
}

pub type Nal = ffi::x264_nal_t;

pub struct EncodeResult<'a> {
    pub nals: &'a [Nal],
    pub pic: *mut Picture,
}

pub struct Encoder {
    h: *mut ffi::x264_t,
}
impl Encoder {
    pub fn open(param: &mut Param) -> Encoder {
        let handle = unsafe { ffi::x264_encoder_open_148(&mut param.raw) };
        Encoder { h: handle }
    }

    pub fn encode(&mut self, pic_in: &mut Picture) -> Result<EncodeResult, isize> {
        let mut p_nal: *mut Nal = unsafe { mem::zeroed() };
        let mut i_nal: c_int = 0;
        let pic_out: &mut Picture = unsafe { mem::zeroed() };
        let ret = unsafe {
            ffi::x264_encoder_encode(self.h,
                                     &mut p_nal,
                                     &mut i_nal,
                                     &mut pic_in.raw,
                                     &mut pic_out.raw)
        } as isize;
        if ret < 0 {
            Err(ret)
        } else {
            let r = EncodeResult {
                nals: unsafe { mem::transmute((p_nal, i_nal as isize)) },
                pic: pic_out,
            };
            Ok(r)
        }
    }
}
impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe { ffi::x264_encoder_close(self.h) }
    }
}
