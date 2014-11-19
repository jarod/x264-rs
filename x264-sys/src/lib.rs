#![allow(bad_style)]

extern crate libc;

use libc::{int64_t,uint8_t,uint32_t};
pub type __builtin_va_list = [__va_list_tag, ..1u];
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
pub type __va_list_tag = Struct___va_list_tag;

#[repr(C)]
pub struct Struct___va_list_tag {
    pub gp_offset: ::libc::c_uint,
    pub fp_offset: ::libc::c_uint,
    pub overflow_arg_area: *mut ::libc::c_void,
    pub reg_save_area: *mut ::libc::c_void,
}

pub enum Struct_x264_t { }
pub type x264_t = Struct_x264_t;
pub type Enum_nal_unit_type_e = ::libc::c_uint;
pub const NAL_UNKNOWN: ::libc::c_uint = 0;
pub const NAL_SLICE: ::libc::c_uint = 1;
pub const NAL_SLICE_DPA: ::libc::c_uint = 2;
pub const NAL_SLICE_DPB: ::libc::c_uint = 3;
pub const NAL_SLICE_DPC: ::libc::c_uint = 4;
pub const NAL_SLICE_IDR: ::libc::c_uint = 5;
pub const NAL_SEI: ::libc::c_uint = 6;
pub const NAL_SPS: ::libc::c_uint = 7;
pub const NAL_PPS: ::libc::c_uint = 8;
pub const NAL_AUD: ::libc::c_uint = 9;
pub const NAL_FILLER: ::libc::c_uint = 12;
pub type Enum_nal_priority_e = ::libc::c_uint;
pub const NAL_PRIORITY_DISPOSABLE: ::libc::c_uint = 0;
pub const NAL_PRIORITY_LOW: ::libc::c_uint = 1;
pub const NAL_PRIORITY_HIGH: ::libc::c_uint = 2;
pub const NAL_PRIORITY_HIGHEST: ::libc::c_uint = 3;
#[repr(C)]
pub struct Struct_Unnamed1 {
    pub i_ref_idc: ::libc::c_int,
    pub i_type: ::libc::c_int,
    pub b_long_startcode: ::libc::c_int,
    pub i_first_mb: ::libc::c_int,
    pub i_last_mb: ::libc::c_int,
    pub i_payload: ::libc::c_int,
    pub p_payload: *mut uint8_t,
    pub i_padding: ::libc::c_int,
}
pub type x264_nal_t = Struct_Unnamed1;
#[repr(C)]
pub struct Struct_Unnamed2 {
    pub i_start: ::libc::c_int,
    pub i_end: ::libc::c_int,
    pub b_force_qp: ::libc::c_int,
    pub i_qp: ::libc::c_int,
    pub f_bitrate_factor: ::libc::c_float,
    pub param: *mut Struct_x264_param_t,
}
pub type x264_zone_t = Struct_Unnamed2;
#[repr(C)]
pub struct Struct_x264_param_t {
    pub cpu: ::libc::c_uint,
    pub i_threads: ::libc::c_int,
    pub i_lookahead_threads: ::libc::c_int,
    pub b_sliced_threads: ::libc::c_int,
    pub b_deterministic: ::libc::c_int,
    pub b_cpu_independent: ::libc::c_int,
    pub i_sync_lookahead: ::libc::c_int,
    pub i_width: ::libc::c_int,
    pub i_height: ::libc::c_int,
    pub i_csp: ::libc::c_int,
    pub i_level_idc: ::libc::c_int,
    pub i_frame_total: ::libc::c_int,
    pub i_nal_hrd: ::libc::c_int,
    pub vui: Struct_Unnamed3,
    pub i_frame_reference: ::libc::c_int,
    pub i_dpb_size: ::libc::c_int,
    pub i_keyint_max: ::libc::c_int,
    pub i_keyint_min: ::libc::c_int,
    pub i_scenecut_threshold: ::libc::c_int,
    pub b_intra_refresh: ::libc::c_int,
    pub i_bframe: ::libc::c_int,
    pub i_bframe_adaptive: ::libc::c_int,
    pub i_bframe_bias: ::libc::c_int,
    pub i_bframe_pyramid: ::libc::c_int,
    pub b_open_gop: ::libc::c_int,
    pub b_bluray_compat: ::libc::c_int,
    pub i_avcintra_class: ::libc::c_int,
    pub b_deblocking_filter: ::libc::c_int,
    pub i_deblocking_filter_alphac0: ::libc::c_int,
    pub i_deblocking_filter_beta: ::libc::c_int,
    pub b_cabac: ::libc::c_int,
    pub i_cabac_init_idc: ::libc::c_int,
    pub b_interlaced: ::libc::c_int,
    pub b_constrained_intra: ::libc::c_int,
    pub i_cqm_preset: ::libc::c_int,
    pub psz_cqm_file: *mut ::libc::c_char,
    pub cqm_4iy: [uint8_t, ..16u],
    pub cqm_4py: [uint8_t, ..16u],
    pub cqm_4ic: [uint8_t, ..16u],
    pub cqm_4pc: [uint8_t, ..16u],
    pub cqm_8iy: [uint8_t, ..64u],
    pub cqm_8py: [uint8_t, ..64u],
    pub cqm_8ic: [uint8_t, ..64u],
    pub cqm_8pc: [uint8_t, ..64u],
    pub pf_log: ::std::option::Option<extern "C" fn
                                          (arg1: *mut ::libc::c_void,
                                           arg2: ::libc::c_int,
                                           arg3: *const ::libc::c_char,
                                           arg4: va_list)>,
    pub p_log_private: *mut ::libc::c_void,
    pub i_log_level: ::libc::c_int,
    pub b_full_recon: ::libc::c_int,
    pub psz_dump_yuv: *mut ::libc::c_char,
    pub analyse: Struct_Unnamed4,
    pub rc: Struct_Unnamed5,
    pub crop_rect: Struct_Unnamed6,
    pub i_frame_packing: ::libc::c_int,
    pub b_aud: ::libc::c_int,
    pub b_repeat_headers: ::libc::c_int,
    pub b_annexb: ::libc::c_int,
    pub i_sps_id: ::libc::c_int,
    pub b_vfr_input: ::libc::c_int,
    pub b_pulldown: ::libc::c_int,
    pub i_fps_num: uint32_t,
    pub i_fps_den: uint32_t,
    pub i_timebase_num: uint32_t,
    pub i_timebase_den: uint32_t,
    pub b_tff: ::libc::c_int,
    pub b_pic_struct: ::libc::c_int,
    pub b_fake_interlaced: ::libc::c_int,
    pub b_stitchable: ::libc::c_int,
    pub b_opencl: ::libc::c_int,
    pub i_opencl_device: ::libc::c_int,
    pub opencl_device_id: *mut ::libc::c_void,
    pub psz_clbin_file: *mut ::libc::c_char,
    pub i_slice_max_size: ::libc::c_int,
    pub i_slice_max_mbs: ::libc::c_int,
    pub i_slice_min_mbs: ::libc::c_int,
    pub i_slice_count: ::libc::c_int,
    pub i_slice_count_max: ::libc::c_int,
    pub param_free: ::std::option::Option<extern "C" fn
                                              (arg1: *mut ::libc::c_void)>,
    pub nalu_process: ::std::option::Option<extern "C" fn
                                                (arg1: *mut x264_t,
                                                 arg2: *mut x264_nal_t,
                                                 arg3: *mut ::libc::c_void)>,
}
#[repr(C)]
pub struct Struct_Unnamed3 {
    pub i_sar_height: ::libc::c_int,
    pub i_sar_width: ::libc::c_int,
    pub i_overscan: ::libc::c_int,
    pub i_vidformat: ::libc::c_int,
    pub b_fullrange: ::libc::c_int,
    pub i_colorprim: ::libc::c_int,
    pub i_transfer: ::libc::c_int,
    pub i_colmatrix: ::libc::c_int,
    pub i_chroma_loc: ::libc::c_int,
}
#[repr(C)]
pub struct Struct_Unnamed4 {
    pub intra: ::libc::c_uint,
    pub inter: ::libc::c_uint,
    pub b_transform_8x8: ::libc::c_int,
    pub i_weighted_pred: ::libc::c_int,
    pub b_weighted_bipred: ::libc::c_int,
    pub i_direct_mv_pred: ::libc::c_int,
    pub i_chroma_qp_offset: ::libc::c_int,
    pub i_me_method: ::libc::c_int,
    pub i_me_range: ::libc::c_int,
    pub i_mv_range: ::libc::c_int,
    pub i_mv_range_thread: ::libc::c_int,
    pub i_subpel_refine: ::libc::c_int,
    pub b_chroma_me: ::libc::c_int,
    pub b_mixed_references: ::libc::c_int,
    pub i_trellis: ::libc::c_int,
    pub b_fast_pskip: ::libc::c_int,
    pub b_dct_decimate: ::libc::c_int,
    pub i_noise_reduction: ::libc::c_int,
    pub f_psy_rd: ::libc::c_float,
    pub f_psy_trellis: ::libc::c_float,
    pub b_psy: ::libc::c_int,
    pub b_mb_info: ::libc::c_int,
    pub b_mb_info_update: ::libc::c_int,
    pub i_luma_deadzone: [::libc::c_int, ..2u],
    pub b_psnr: ::libc::c_int,
    pub b_ssim: ::libc::c_int,
}
#[repr(C)]
pub struct Struct_Unnamed5 {
    pub i_rc_method: ::libc::c_int,
    pub i_qp_constant: ::libc::c_int,
    pub i_qp_min: ::libc::c_int,
    pub i_qp_max: ::libc::c_int,
    pub i_qp_step: ::libc::c_int,
    pub i_bitrate: ::libc::c_int,
    pub f_rf_constant: ::libc::c_float,
    pub f_rf_constant_max: ::libc::c_float,
    pub f_rate_tolerance: ::libc::c_float,
    pub i_vbv_max_bitrate: ::libc::c_int,
    pub i_vbv_buffer_size: ::libc::c_int,
    pub f_vbv_buffer_init: ::libc::c_float,
    pub f_ip_factor: ::libc::c_float,
    pub f_pb_factor: ::libc::c_float,
    pub b_filler: ::libc::c_int,
    pub i_aq_mode: ::libc::c_int,
    pub f_aq_strength: ::libc::c_float,
    pub b_mb_tree: ::libc::c_int,
    pub i_lookahead: ::libc::c_int,
    pub b_stat_write: ::libc::c_int,
    pub psz_stat_out: *mut ::libc::c_char,
    pub b_stat_read: ::libc::c_int,
    pub psz_stat_in: *mut ::libc::c_char,
    pub f_qcompress: ::libc::c_float,
    pub f_qblur: ::libc::c_float,
    pub f_complexity_blur: ::libc::c_float,
    pub zones: *mut x264_zone_t,
    pub i_zones: ::libc::c_int,
    pub psz_zones: *mut ::libc::c_char,
}
#[repr(C)]
pub struct Struct_Unnamed6 {
    pub i_left: ::libc::c_uint,
    pub i_top: ::libc::c_uint,
    pub i_right: ::libc::c_uint,
    pub i_bottom: ::libc::c_uint,
}
pub type x264_param_t = Struct_x264_param_t;
#[repr(C)]
pub struct Struct_Unnamed7 {
    pub level_idc: ::libc::c_int,
    pub mbps: ::libc::c_int,
    pub frame_size: ::libc::c_int,
    pub dpb: ::libc::c_int,
    pub bitrate: ::libc::c_int,
    pub cpb: ::libc::c_int,
    pub mv_range: ::libc::c_int,
    pub mvs_per_2mb: ::libc::c_int,
    pub slice_rate: ::libc::c_int,
    pub mincr: ::libc::c_int,
    pub bipred8x8: ::libc::c_int,
    pub direct8x8: ::libc::c_int,
    pub frame_only: ::libc::c_int,
}
pub type x264_level_t = Struct_Unnamed7;
pub type Enum_pic_struct_e = ::libc::c_uint;
pub const PIC_STRUCT_AUTO: ::libc::c_uint = 0;
pub const PIC_STRUCT_PROGRESSIVE: ::libc::c_uint = 1;
pub const PIC_STRUCT_TOP_BOTTOM: ::libc::c_uint = 4;
pub const PIC_STRUCT_BOTTOM_TOP: ::libc::c_uint = 5;
pub const PIC_STRUCT_TOP_BOTTOM_TOP: ::libc::c_uint = 6;
pub const PIC_STRUCT_BOTTOM_TOP_BOTTOM: ::libc::c_uint = 7;
pub const PIC_STRUCT_DOUBLE: ::libc::c_uint = 8;
pub const PIC_STRUCT_TRIPLE: ::libc::c_uint = 9;
#[repr(C)]
pub struct Struct_Unnamed8 {
    pub cpb_initial_arrival_time: ::libc::c_double,
    pub cpb_final_arrival_time: ::libc::c_double,
    pub cpb_removal_time: ::libc::c_double,
    pub dpb_output_time: ::libc::c_double,
}
pub type x264_hrd_t = Struct_Unnamed8;
#[repr(C)]
pub struct Struct_Unnamed9 {
    pub payload_size: ::libc::c_int,
    pub payload_type: ::libc::c_int,
    pub payload: *mut uint8_t,
}
pub type x264_sei_payload_t = Struct_Unnamed9;
#[repr(C)]
pub struct Struct_Unnamed10 {
    pub num_payloads: ::libc::c_int,
    pub payloads: *mut x264_sei_payload_t,
    pub sei_free: ::std::option::Option<extern "C" fn
                                            (arg1: *mut ::libc::c_void)>,
}
pub type x264_sei_t = Struct_Unnamed10;
#[repr(C)]
pub struct Struct_Unnamed11 {
    pub i_csp: ::libc::c_int,
    pub i_plane: ::libc::c_int,
    pub i_stride: [::libc::c_int, ..4u],
    pub plane: [*mut uint8_t, ..4u],
}
pub type x264_image_t = Struct_Unnamed11;
#[repr(C)]
pub struct Struct_Unnamed12 {
    pub quant_offsets: *mut ::libc::c_float,
    pub quant_offsets_free: ::std::option::Option<extern "C" fn
                                                      (arg1:
                                                           *mut ::libc::c_void)>,
    pub mb_info: *mut uint8_t,
    pub mb_info_free: ::std::option::Option<extern "C" fn
                                                (arg1: *mut ::libc::c_void)>,
    pub f_ssim: ::libc::c_double,
    pub f_psnr_avg: ::libc::c_double,
    pub f_psnr: [::libc::c_double, ..3u],
    pub f_crf_avg: ::libc::c_double,
}
pub type x264_image_properties_t = Struct_Unnamed12;
#[repr(C)]
pub struct Struct_Unnamed13 {
    pub i_type: ::libc::c_int,
    pub i_qpplus1: ::libc::c_int,
    pub i_pic_struct: ::libc::c_int,
    pub b_keyframe: ::libc::c_int,
    pub i_pts: int64_t,
    pub i_dts: int64_t,
    pub param: *mut x264_param_t,
    pub img: x264_image_t,
    pub prop: x264_image_properties_t,
    pub hrd_timing: x264_hrd_t,
    pub extra_sei: x264_sei_t,
    pub opaque: *mut ::libc::c_void,
}
pub type x264_picture_t = Struct_Unnamed13;
#[link(name = "x264")]
extern "C" {
    pub static mut x264_levels: *const x264_level_t;
    pub static x264_bit_depth: ::libc::c_int;
    pub static x264_chroma_format: ::libc::c_int;
}
#[link(name = "x264")]
extern "C" {
    pub fn x264_nal_encode(h: *mut x264_t, dst: *mut uint8_t,
                           nal: *mut x264_nal_t);
    pub fn x264_param_default(arg1: *mut x264_param_t);
    pub fn x264_param_parse(arg1: *mut x264_param_t,
                            name: *const ::libc::c_char,
                            value: *const ::libc::c_char) -> ::libc::c_int;
    pub fn x264_param_default_preset(arg1: *mut x264_param_t,
                                     preset: *const ::libc::c_char,
                                     tune: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn x264_param_apply_fastfirstpass(arg1: *mut x264_param_t);
    pub fn x264_param_apply_profile(arg1: *mut x264_param_t,
                                    profile: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn x264_picture_init(pic: *mut x264_picture_t);
    pub fn x264_picture_alloc(pic: *mut x264_picture_t, i_csp: ::libc::c_int,
                              i_width: ::libc::c_int, i_height: ::libc::c_int)
     -> ::libc::c_int;
    pub fn x264_picture_clean(pic: *mut x264_picture_t);
    pub fn x264_encoder_open_142(arg1: *mut x264_param_t) -> *mut x264_t;
    pub fn x264_encoder_reconfig(arg1: *mut x264_t, arg2: *mut x264_param_t)
     -> ::libc::c_int;
    pub fn x264_encoder_parameters(arg1: *mut x264_t,
                                   arg2: *mut x264_param_t);
    pub fn x264_encoder_headers(arg1: *mut x264_t,
                                pp_nal: *mut *mut x264_nal_t,
                                pi_nal: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn x264_encoder_encode(arg1: *mut x264_t,
                               pp_nal: *mut *mut x264_nal_t,
                               pi_nal: *mut ::libc::c_int,
                               pic_in: *mut x264_picture_t,
                               pic_out: *mut x264_picture_t) -> ::libc::c_int;
    pub fn x264_encoder_close(arg1: *mut x264_t);
    pub fn x264_encoder_delayed_frames(arg1: *mut x264_t) -> ::libc::c_int;
    pub fn x264_encoder_maximum_delayed_frames(h: *mut x264_t)
     -> ::libc::c_int;
    pub fn x264_encoder_intra_refresh(arg1: *mut x264_t);
    pub fn x264_encoder_invalidate_reference(arg1: *mut x264_t, pts: int64_t)
     -> ::libc::c_int;
}
