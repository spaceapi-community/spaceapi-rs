extern crate syntex;
extern crate serde_codegen;

use std::env;
use std::path::Path;

pub fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let src_sensors = Path::new("src/sensors.rs.in");
    let dst_sensors = Path::new(&out_dir).join("sensors.rs");
    
    let src_status = Path::new("src/status.rs.in");
    let dst_status = Path::new(&out_dir).join("status.rs");

    let mut registry = syntex::Registry::new();
    serde_codegen::register(&mut registry);
    registry.expand("", &src_sensors, &dst_sensors).unwrap();

    let mut registry = syntex::Registry::new();
    serde_codegen::register(&mut registry);
    registry.expand("", &src_status, &dst_status).unwrap();
}
