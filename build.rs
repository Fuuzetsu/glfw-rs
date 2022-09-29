extern crate pkg_config;

fn main() {
    pkg_config::probe_library("glfw3").unwrap();
    pkg_config::probe_library("x11").unwrap();
}
