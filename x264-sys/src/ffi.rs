/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type int_least8_t = ::std::os::raw::c_char;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_long;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulong;
pub type int_fast8_t = ::std::os::raw::c_char;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
pub enum x264_t { }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nal_unit_type_e {
    NAL_UNKNOWN = 0,
    NAL_SLICE = 1,
    NAL_SLICE_DPA = 2,
    NAL_SLICE_DPB = 3,
    NAL_SLICE_DPC = 4,
    NAL_SLICE_IDR = 5,
    NAL_SEI = 6,
    NAL_SPS = 7,
    NAL_PPS = 8,
    NAL_AUD = 9,
    NAL_FILLER = 12,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nal_priority_e {
    NAL_PRIORITY_DISPOSABLE = 0,
    NAL_PRIORITY_LOW = 1,
    NAL_PRIORITY_HIGH = 2,
    NAL_PRIORITY_HIGHEST = 3,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct x264_nal_t {
    pub i_ref_idc: ::std::os::raw::c_int,
    pub i_type: ::std::os::raw::c_int,
    pub b_long_startcode: ::std::os::raw::c_int,
    pub i_first_mb: ::std::os::raw::c_int,
    pub i_last_mb: ::std::os::raw::c_int,
    pub i_payload: ::std::os::raw::c_int,
    pub p_payload: *mut uint8_t,
    pub i_padding: ::std::os::raw::c_int,
}
impl ::std::default::Default for x264_nal_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct x264_zone_t {
    pub i_start: ::std::os::raw::c_int,
    pub i_end: ::std::os::raw::c_int,
    pub b_force_qp: ::std::os::raw::c_int,
    pub i_qp: ::std::os::raw::c_int,
    pub f_bitrate_factor: f32,
    pub param: *mut x264_param_t,
}
impl ::std::default::Default for x264_zone_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct x264_param_t {
    pub cpu: ::std::os::raw::c_uint,
    pub i_threads: ::std::os::raw::c_int,
    pub i_lookahead_threads: ::std::os::raw::c_int,
    pub b_sliced_threads: ::std::os::raw::c_int,
    pub b_deterministic: ::std::os::raw::c_int,
    pub b_cpu_independent: ::std::os::raw::c_int,
    pub i_sync_lookahead: ::std::os::raw::c_int,
    pub i_width: ::std::os::raw::c_int,
    pub i_height: ::std::os::raw::c_int,
    pub i_csp: ::std::os::raw::c_int,
    pub i_level_idc: ::std::os::raw::c_int,
    pub i_frame_total: ::std::os::raw::c_int,
    pub i_nal_hrd: ::std::os::raw::c_int,
    pub vui: Struct_Unnamed1,
    pub i_frame_reference: ::std::os::raw::c_int,
    pub i_dpb_size: ::std::os::raw::c_int,
    pub i_keyint_max: ::std::os::raw::c_int,
    pub i_keyint_min: ::std::os::raw::c_int,
    pub i_scenecut_threshold: ::std::os::raw::c_int,
    pub b_intra_refresh: ::std::os::raw::c_int,
    pub i_bframe: ::std::os::raw::c_int,
    pub i_bframe_adaptive: ::std::os::raw::c_int,
    pub i_bframe_bias: ::std::os::raw::c_int,
    pub i_bframe_pyramid: ::std::os::raw::c_int,
    pub b_open_gop: ::std::os::raw::c_int,
    pub b_bluray_compat: ::std::os::raw::c_int,
    pub i_avcintra_class: ::std::os::raw::c_int,
    pub b_deblocking_filter: ::std::os::raw::c_int,
    pub i_deblocking_filter_alphac0: ::std::os::raw::c_int,
    pub i_deblocking_filter_beta: ::std::os::raw::c_int,
    pub b_cabac: ::std::os::raw::c_int,
    pub i_cabac_init_idc: ::std::os::raw::c_int,
    pub b_interlaced: ::std::os::raw::c_int,
    pub b_constrained_intra: ::std::os::raw::c_int,
    pub i_cqm_preset: ::std::os::raw::c_int,
    pub psz_cqm_file: *mut ::std::os::raw::c_char,
    pub cqm_4iy: [uint8_t; 16usize],
    pub cqm_4py: [uint8_t; 16usize],
    pub cqm_4ic: [uint8_t; 16usize],
    pub cqm_4pc: [uint8_t; 16usize],
    pub cqm_8iy: [uint8_t; 64usize],
    pub cqm_8py: [uint8_t; 64usize],
    pub cqm_8ic: [uint8_t; 64usize],
    pub cqm_8pc: [uint8_t; 64usize],
    pub pf_log: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                               *mut ::std::os::raw::c_void,
                                                           i_level:
                                                               ::std::os::raw::c_int,
                                                           psz:
                                                               *const ::std::os::raw::c_char,
                                                           arg2: va_list)>,
    pub p_log_private: *mut ::std::os::raw::c_void,
    pub i_log_level: ::std::os::raw::c_int,
    pub b_full_recon: ::std::os::raw::c_int,
    pub psz_dump_yuv: *mut ::std::os::raw::c_char,
    pub analyse: Struct_Unnamed2,
    pub rc: Struct_Unnamed3,
    pub crop_rect: Struct_Unnamed4,
    pub i_frame_packing: ::std::os::raw::c_int,
    pub b_aud: ::std::os::raw::c_int,
    pub b_repeat_headers: ::std::os::raw::c_int,
    pub b_annexb: ::std::os::raw::c_int,
    pub i_sps_id: ::std::os::raw::c_int,
    pub b_vfr_input: ::std::os::raw::c_int,
    pub b_pulldown: ::std::os::raw::c_int,
    pub i_fps_num: uint32_t,
    pub i_fps_den: uint32_t,
    pub i_timebase_num: uint32_t,
    pub i_timebase_den: uint32_t,
    pub b_tff: ::std::os::raw::c_int,
    pub b_pic_struct: ::std::os::raw::c_int,
    pub b_fake_interlaced: ::std::os::raw::c_int,
    pub b_stitchable: ::std::os::raw::c_int,
    pub b_opencl: ::std::os::raw::c_int,
    pub i_opencl_device: ::std::os::raw::c_int,
    pub opencl_device_id: *mut ::std::os::raw::c_void,
    pub psz_clbin_file: *mut ::std::os::raw::c_char,
    pub i_slice_max_size: ::std::os::raw::c_int,
    pub i_slice_max_mbs: ::std::os::raw::c_int,
    pub i_slice_min_mbs: ::std::os::raw::c_int,
    pub i_slice_count: ::std::os::raw::c_int,
    pub i_slice_count_max: ::std::os::raw::c_int,
    pub param_free: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                   *mut ::std::os::raw::c_void)>,
    pub nalu_process: ::std::option::Option<unsafe extern "C" fn(h:
                                                                     *mut x264_t,
                                                                 nal:
                                                                     *mut x264_nal_t,
                                                                 opaque:
                                                                     *mut ::std::os::raw::c_void)>,
}
impl ::std::clone::Clone for x264_param_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for x264_param_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Struct_Unnamed1 {
    pub i_sar_height: ::std::os::raw::c_int,
    pub i_sar_width: ::std::os::raw::c_int,
    pub i_overscan: ::std::os::raw::c_int,
    pub i_vidformat: ::std::os::raw::c_int,
    pub b_fullrange: ::std::os::raw::c_int,
    pub i_colorprim: ::std::os::raw::c_int,
    pub i_transfer: ::std::os::raw::c_int,
    pub i_colmatrix: ::std::os::raw::c_int,
    pub i_chroma_loc: ::std::os::raw::c_int,
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Struct_Unnamed2 {
    pub intra: ::std::os::raw::c_uint,
    pub inter: ::std::os::raw::c_uint,
    pub b_transform_8x8: ::std::os::raw::c_int,
    pub i_weighted_pred: ::std::os::raw::c_int,
    pub b_weighted_bipred: ::std::os::raw::c_int,
    pub i_direct_mv_pred: ::std::os::raw::c_int,
    pub i_chroma_qp_offset: ::std::os::raw::c_int,
    pub i_me_method: ::std::os::raw::c_int,
    pub i_me_range: ::std::os::raw::c_int,
    pub i_mv_range: ::std::os::raw::c_int,
    pub i_mv_range_thread: ::std::os::raw::c_int,
    pub i_subpel_refine: ::std::os::raw::c_int,
    pub b_chroma_me: ::std::os::raw::c_int,
    pub b_mixed_references: ::std::os::raw::c_int,
    pub i_trellis: ::std::os::raw::c_int,
    pub b_fast_pskip: ::std::os::raw::c_int,
    pub b_dct_decimate: ::std::os::raw::c_int,
    pub i_noise_reduction: ::std::os::raw::c_int,
    pub f_psy_rd: f32,
    pub f_psy_trellis: f32,
    pub b_psy: ::std::os::raw::c_int,
    pub b_mb_info: ::std::os::raw::c_int,
    pub b_mb_info_update: ::std::os::raw::c_int,
    pub i_luma_deadzone: [::std::os::raw::c_int; 2usize],
    pub b_psnr: ::std::os::raw::c_int,
    pub b_ssim: ::std::os::raw::c_int,
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Struct_Unnamed3 {
    pub i_rc_method: ::std::os::raw::c_int,
    pub i_qp_constant: ::std::os::raw::c_int,
    pub i_qp_min: ::std::os::raw::c_int,
    pub i_qp_max: ::std::os::raw::c_int,
    pub i_qp_step: ::std::os::raw::c_int,
    pub i_bitrate: ::std::os::raw::c_int,
    pub f_rf_constant: f32,
    pub f_rf_constant_max: f32,
    pub f_rate_tolerance: f32,
    pub i_vbv_max_bitrate: ::std::os::raw::c_int,
    pub i_vbv_buffer_size: ::std::os::raw::c_int,
    pub f_vbv_buffer_init: f32,
    pub f_ip_factor: f32,
    pub f_pb_factor: f32,
    pub b_filler: ::std::os::raw::c_int,
    pub i_aq_mode: ::std::os::raw::c_int,
    pub f_aq_strength: f32,
    pub b_mb_tree: ::std::os::raw::c_int,
    pub i_lookahead: ::std::os::raw::c_int,
    pub b_stat_write: ::std::os::raw::c_int,
    pub psz_stat_out: *mut ::std::os::raw::c_char,
    pub b_stat_read: ::std::os::raw::c_int,
    pub psz_stat_in: *mut ::std::os::raw::c_char,
    pub f_qcompress: f32,
    pub f_qblur: f32,
    pub f_complexity_blur: f32,
    pub zones: *mut x264_zone_t,
    pub i_zones: ::std::os::raw::c_int,
    pub psz_zones: *mut ::std::os::raw::c_char,
}
impl ::std::default::Default for Struct_Unnamed3 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Struct_Unnamed4 {
    pub i_left: ::std::os::raw::c_uint,
    pub i_top: ::std::os::raw::c_uint,
    pub i_right: ::std::os::raw::c_uint,
    pub i_bottom: ::std::os::raw::c_uint,
}
impl ::std::default::Default for Struct_Unnamed4 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct x264_level_t {
    pub level_idc: ::std::os::raw::c_int,
    pub mbps: ::std::os::raw::c_int,
    pub frame_size: ::std::os::raw::c_int,
    pub dpb: ::std::os::raw::c_int,
    pub bitrate: ::std::os::raw::c_int,
    pub cpb: ::std::os::raw::c_int,
    pub mv_range: ::std::os::raw::c_int,
    pub mvs_per_2mb: ::std::os::raw::c_int,
    pub slice_rate: ::std::os::raw::c_int,
    pub mincr: ::std::os::raw::c_int,
    pub bipred8x8: ::std::os::raw::c_int,
    pub direct8x8: ::std::os::raw::c_int,
    pub frame_only: ::std::os::raw::c_int,
}
impl ::std::default::Default for x264_level_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum pic_struct_e {
    PIC_STRUCT_AUTO = 0,
    PIC_STRUCT_PROGRESSIVE = 1,
    PIC_STRUCT_TOP_BOTTOM = 4,
    PIC_STRUCT_BOTTOM_TOP = 5,
    PIC_STRUCT_TOP_BOTTOM_TOP = 6,
    PIC_STRUCT_BOTTOM_TOP_BOTTOM = 7,
    PIC_STRUCT_DOUBLE = 8,
    PIC_STRUCT_TRIPLE = 9,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct x264_hrd_t {
    pub cpb_initial_arrival_time: f64,
    pub cpb_final_arrival_time: f64,
    pub cpb_removal_time: f64,
    pub dpb_output_time: f64,
}
impl ::std::default::Default for x264_hrd_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct x264_sei_payload_t {
    pub payload_size: ::std::os::raw::c_int,
    pub payload_type: ::std::os::raw::c_int,
    pub payload: *mut uint8_t,
}
impl ::std::default::Default for x264_sei_payload_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct x264_sei_t {
    pub num_payloads: ::std::os::raw::c_int,
    pub payloads: *mut x264_sei_payload_t,
    pub sei_free: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                 *mut ::std::os::raw::c_void)>,
}
impl ::std::default::Default for x264_sei_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct x264_image_t {
    pub i_csp: ::std::os::raw::c_int,
    pub i_plane: ::std::os::raw::c_int,
    pub i_stride: [::std::os::raw::c_int; 4usize],
    pub plane: [*mut uint8_t; 4usize],
}
impl ::std::default::Default for x264_image_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct x264_image_properties_t {
    pub quant_offsets: *mut f32,
    pub quant_offsets_free: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                           *mut ::std::os::raw::c_void)>,
    pub mb_info: *mut uint8_t,
    pub mb_info_free: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                     *mut ::std::os::raw::c_void)>,
    pub f_ssim: f64,
    pub f_psnr_avg: f64,
    pub f_psnr: [f64; 3usize],
    pub f_crf_avg: f64,
}
impl ::std::default::Default for x264_image_properties_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct x264_picture_t {
    pub i_type: ::std::os::raw::c_int,
    pub i_qpplus1: ::std::os::raw::c_int,
    pub i_pic_struct: ::std::os::raw::c_int,
    pub b_keyframe: ::std::os::raw::c_int,
    pub i_pts: int64_t,
    pub i_dts: int64_t,
    pub param: *mut x264_param_t,
    pub img: x264_image_t,
    pub prop: x264_image_properties_t,
    pub hrd_timing: x264_hrd_t,
    pub extra_sei: x264_sei_t,
    pub opaque: *mut ::std::os::raw::c_void,
}
impl ::std::default::Default for x264_picture_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
impl ::std::default::Default for __va_list_tag {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub static mut x264_levels: [x264_level_t; 0usize];
    pub static x264_bit_depth: ::std::os::raw::c_int;
    pub static x264_chroma_format: ::std::os::raw::c_int;
}
extern "C" {
    pub fn x264_nal_encode(h: *mut x264_t, dst: *mut uint8_t,
                           nal: *mut x264_nal_t);
    pub fn x264_param_default(arg1: *mut x264_param_t);
    pub fn x264_param_parse(arg1: *mut x264_param_t,
                            name: *const ::std::os::raw::c_char,
                            value: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn x264_param_default_preset(arg1: *mut x264_param_t,
                                     preset: *const ::std::os::raw::c_char,
                                     tune: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn x264_param_apply_fastfirstpass(arg1: *mut x264_param_t);
    pub fn x264_param_apply_profile(arg1: *mut x264_param_t,
                                    profile: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn x264_picture_init(pic: *mut x264_picture_t);
    pub fn x264_picture_alloc(pic: *mut x264_picture_t,
                              i_csp: ::std::os::raw::c_int,
                              i_width: ::std::os::raw::c_int,
                              i_height: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn x264_picture_clean(pic: *mut x264_picture_t);
    pub fn x264_encoder_open_148(arg1: *mut x264_param_t) -> *mut x264_t;
    pub fn x264_encoder_reconfig(arg1: *mut x264_t, arg2: *mut x264_param_t)
     -> ::std::os::raw::c_int;
    pub fn x264_encoder_parameters(arg1: *mut x264_t,
                                   arg2: *mut x264_param_t);
    pub fn x264_encoder_headers(arg1: *mut x264_t,
                                pp_nal: *mut *mut x264_nal_t,
                                pi_nal: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn x264_encoder_encode(arg1: *mut x264_t,
                               pp_nal: *mut *mut x264_nal_t,
                               pi_nal: *mut ::std::os::raw::c_int,
                               pic_in: *mut x264_picture_t,
                               pic_out: *mut x264_picture_t)
     -> ::std::os::raw::c_int;
    pub fn x264_encoder_close(arg1: *mut x264_t);
    pub fn x264_encoder_delayed_frames(arg1: *mut x264_t)
     -> ::std::os::raw::c_int;
    pub fn x264_encoder_maximum_delayed_frames(h: *mut x264_t)
     -> ::std::os::raw::c_int;
    pub fn x264_encoder_intra_refresh(arg1: *mut x264_t);
    pub fn x264_encoder_invalidate_reference(arg1: *mut x264_t, pts: int64_t)
     -> ::std::os::raw::c_int;
}
