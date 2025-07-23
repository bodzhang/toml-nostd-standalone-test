#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::{String, ToString};
use core::panic::PanicInfo;
use serde::{Deserialize, Serialize};

// Note: We intentionally try to use toml here to see if it compiles
// If toml v0.9.2 uses f64::copysign(), this should fail to compile
// in no_std environment because copysign is not available
use toml;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,
    version: String,
    pi: f64,
    negative_value: f64,
    zero: f64,
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // This is where we test TOML serialization in no_std environment
    let config = Config {
        name: "test".to_string(),
        version: "1.0.0".to_string(),
        pi: 3.141592653589793,
        negative_value: -42.5,
        zero: 0.0,
    };

    // This should fail to compile if f64::copysign() is used internally by toml
    // because copysign is not available in no_std environments without libm
    let _toml_string = toml::to_string(&config);

    // If we reach here, it means the compilation succeeded
    // which would contradict the claim about f64::copysign()
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
