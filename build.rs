#[allow(non_snake_case)]
extern crate bindgen;

use std::env;
use std::path::PathBuf;
//use std::process::Command;

use std::collections::HashSet;

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn main() {
    // Tell cargo to tell rustc to link the v5rt library
    // shared library
    /*println!("cargo:rustc-link-search=libraries/");
    println!("cargo:rustc-link-search=/usr/lib/gcc/arm-none-eabi/7.3.1/hard/");
    println!("cargo:rustc-link-search=/usr/arm-none-eabi/lib");
    //println!("cargo:rustc-link-search=/usr/lib/arm-none-eabi/newlib/");
    println!("cargo:rustc-link-search=/home/drew/stlib/");

    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=m");
    println!("cargo:rustc-link-lib=static=g");
    println!("cargo:rustc-link-lib=static=gcc");
    println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rustc-link-lib=static=supc++");
    println!("cargo:rustc-link-lib=static=pros");
    println!("cargo:rustc-link-lib=static=okapi");*/



    let ignored_macros = IgnoreMacros(
            vec![
                "FP_INFINITE".into(),
                "FP_NAN".into(),
                "FP_NORMAL".into(),
                "FP_SUBNORMAL".into(),
                "FP_ZERO".into(),
                "IPPORT_RESERVED".into(),
            ]
            .into_iter()
            .collect(),
        );

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("include/api.h")
        .header("include/pros/apix.h")
        .clang_arg("-I'$(TOOLCHAIN)/$(PLATFORM)/include'")
        .use_core()
        .ctypes_prefix("cty")
        .parse_callbacks(Box::new(ignored_macros))
        .rustfmt_bindings(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");


    //arm-none-eabi-g++ -mcpu=cortex-a9 -mfpu=neon-fp16 -mfloat-abi=softfp   -nostdlib ./bin/AStar.cc.o ./bin/DStar.cpp.o ./bin/DriveControl.cc.o ./bin/HeightController.cc.o ./bin/IntakeControl.cc.o ./bin/LiftControl.cc.o ./bin/MiniPID.cpp.o ./bin/Tracking.cc.o ./bin/autonomous.cc.o ./bin/initialize.cc.o ./bin/motor.cc.o ./bin/opcontrol.cc.o ./bin/_pros_ld_timestamp.o -Wl,-T./firmware/v5.ld,--gc-sections,--start-group,./firmware/libpros.a,./firmware/okapilib.a,-lc,-lm,-lgcc,-lstdc++,-lsupc++,--end-group -o bin/monolith.elf 2> temp.log || touch temp.errors
}
