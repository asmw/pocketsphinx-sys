extern crate libc;

use libc::{c_int, c_char};

// Dummy structures for typed pointers.
// Only use them as part of pointer type, never dereference.
#[repr(C)] pub struct cmd_ln_t(i32);
#[repr(C)] pub struct arg_t(i32);
#[repr(C)] pub struct ps_decoder_t(i32);

#[link(name="pocketsphinx")]
extern {

    pub fn cmd_ln_parse_r(inout_cmdln: *mut cmd_ln_t,
                          defn: *const arg_t,
                          argc: i32,
                          argv: *const *const c_char,
                          strict: i32, ...)
                          -> *mut cmd_ln_t;
    pub fn cmd_ln_free_r(cmdln: *mut cmd_ln_t) -> i32;


    pub fn ps_args() -> *const arg_t;
    pub fn ps_init(config: *mut cmd_ln_t) -> *mut ps_decoder_t;
    pub fn ps_free(ps: *mut ps_decoder_t) -> c_int;

    pub fn ps_start_utt(ps: *mut ps_decoder_t, uttid: *const c_char) -> c_int;
    pub fn ps_process_raw(ps: *mut ps_decoder_t,
                          data: *const i16,
                          n_samples: usize,
                          no_search: c_int,
                          full_utt: c_int)
                          -> c_int;
    pub fn ps_end_utt(ps: *mut ps_decoder_t) -> c_int;

    pub fn ps_get_hyp(ps: *mut ps_decoder_t,
                      out_best_score: *mut i32,
                      out_uttid: *mut *const c_char)
                      -> *const c_char;

}
