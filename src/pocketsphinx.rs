use libc::{c_int, c_char, c_long, c_double};

use super::glist::anytype_t;

// Dummy structures for typed pointers.
// Only use them as part of pointer type, never dereference.
#[allow(non_camel_case_types)] pub enum cmd_ln_t {}
#[allow(non_camel_case_types)] pub enum ps_decoder_t {}
#[allow(non_camel_case_types)] pub enum ps_nbest_t {}
#[allow(non_camel_case_types)] pub enum ps_seg_t {}

#[repr(C)]
pub struct arg_t {
    name: *const c_char,
    _type: c_int,
    deflt: *const c_char,
    doc: *const c_char,
}

pub const ARG_REQUIRED: c_int = 1 << 0;
pub const ARG_INTEGER: c_int = 1 << 1;
pub const ARG_FLOATING: c_int = 1 << 2;
pub const ARG_STRING: c_int = 1 << 3;
pub const ARG_BOOLEAN: c_int = 1 << 4;
pub const ARG_STRING_LIST: c_int = 1 << 5;

#[link(name="pocketsphinx")]
extern {

    pub fn cmd_ln_parse_r(inout_cmdln: *mut cmd_ln_t,
                          defn: *const arg_t,
                          argc: i32,
                          argv: *const *const c_char,
                          strict: i32, ...)
                          -> *mut cmd_ln_t;
    pub fn cmd_ln_free_r(cmdln: *mut cmd_ln_t) -> i32;
    pub fn cmd_ln_access_r(cmdln: *const cmd_ln_t, name: *const c_char) -> *const anytype_t;
    pub fn cmd_ln_str_r(cmdln: *const cmd_ln_t, name: *const c_char) -> *const c_char;
    pub fn cmd_ln_int_r(cmdln: *const cmd_ln_t, name: *const c_char) -> c_long;
    pub fn cmd_ln_float_r(cmdln: *const cmd_ln_t, name: *const c_char) -> c_double;
    pub fn cmd_ln_exists_r(cmdln: *const cmd_ln_t, name: *const c_char) -> c_int;


    pub fn ps_args() -> *const arg_t;
    pub fn ps_init(config: *mut cmd_ln_t) -> *mut ps_decoder_t;
    pub fn ps_free(ps: *mut ps_decoder_t) -> c_int;

    pub fn ps_get_config(ps: *const ps_decoder_t) -> *const cmd_ln_t;

    pub fn ps_start_utt(ps: *mut ps_decoder_t, uttid: *const c_char) -> c_int;
    pub fn ps_process_raw(ps: *mut ps_decoder_t,
                          data: *const i16,
                          n_samples: usize,
                          no_search: c_int,
                          full_utt: c_int)
                          -> c_int;
    pub fn ps_get_in_speech(ps: *const ps_decoder_t) -> u8;
    pub fn ps_end_utt(ps: *mut ps_decoder_t) -> c_int;

    pub fn ps_get_hyp(ps: *mut ps_decoder_t,
                      out_best_score: *mut i32,
                      out_uttid: *mut *const c_char)
                      -> *const c_char;
    pub fn ps_get_prob(ps: *const ps_decoder_t) -> i32;
    pub fn ps_get_n_frames(ps: *const ps_decoder_t) -> i32;

    pub fn ps_nbest(ps: *const ps_decoder_t, start_frame: c_int, end_frame: c_int,
                    ctx1: *const c_char, ctx2: *const c_char) -> *mut ps_nbest_t;
    pub fn ps_nbest_free(nbest: *mut ps_nbest_t);
    pub fn ps_nbest_hyp(nbest: *const ps_nbest_t, out_score: *mut i32) -> *const c_char;
    pub fn ps_nbest_next(nbest: *mut ps_nbest_t) -> *mut ps_nbest_t;
    pub fn ps_nbest_seg(nbest: *const ps_nbest_t, out_score: *mut i32) -> *mut ps_seg_t;

    pub fn ps_seg_frames(seg: *const ps_seg_t, out_sf: *mut c_int, out_ef: *mut c_int);
    pub fn ps_seg_free(seg: *mut ps_seg_t);
    pub fn ps_seg_iter(ps: *const ps_decoder_t, out_best_score: *mut i32) -> *mut ps_seg_t;
    pub fn ps_seg_next(seg: *mut ps_seg_t) -> *mut ps_seg_t;
    pub fn ps_seg_prob(seg: *const ps_seg_t,
                       out_ascr: *mut i32, out_lscr: *mut i32, out_lback: *mut i32) -> i32;
    pub fn ps_seg_word(seg: *const ps_seg_t) -> *const c_char;

}

// Macros
pub unsafe fn cmd_ln_boolean_r(cmdln: *const cmd_ln_t, name: *const c_char) -> bool {
    cmd_ln_int_r(cmdln, name) != 0
}
pub unsafe fn cmd_ln_int32_r(cmdln: *const cmd_ln_t, name: *const c_char) -> i32 {
    cmd_ln_int_r(cmdln, name) as i32
}
pub unsafe fn cmd_ln_float32_r(cmdln: *const cmd_ln_t, name: *const c_char) -> f32 {
    cmd_ln_float_r(cmdln, name) as f32
}
pub unsafe fn cmd_ln_float64_r(cmdln: *const cmd_ln_t, name: *const c_char) -> f64 {
    cmd_ln_float_r(cmdln, name) as f64
}
