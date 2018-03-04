extern crate pkg_config;

use std::env;

fn main() {
    if !env::var("TARGET").unwrap().contains("msvc") {
        pkg_config::find_library("pocketsphinx").unwrap();
    }
}
