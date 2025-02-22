use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen, get_symbols_if_no_duplicates, RawOpts,
};

const CLASS_NAME: &str = "BtlePlug";
const RUST_INPUT: &str = "src/api.rs";
const DART_OUTPUT: &str = "../lib/bridge_generated.dart";

const IOS_C_OUTPUT: &str = "../ios/Classes/frb.h";
const MACOS_C_OUTPUT: &str = "../macos/Classes/frb.h";

fn main() {
    // Tell Cargo that if the input Rust code changes, rerun this build script
    println!("cargo:rerun-if-changed={}", RUST_INPUT);

    // Options for frb_codegen
    let raw_opts = RawOpts {
        rust_input: vec![RUST_INPUT.to_string()],
        dart_output: vec![DART_OUTPUT.to_string()],
        // c_output: Some(vec![IOS_C_OUTPUT.to_string(), MACOS_C_OUTPUT.to_string()]),
        c_output: Some(vec![IOS_C_OUTPUT.to_string()]),
        class_name: Some(vec![CLASS_NAME.to_string()]),
        inline_rust: true,
        wasm: false,
        ..Default::default()
    };

    // Generate Rust & Dart ffi bridges
    let configs = config_parse(raw_opts);
    let all_symbols = get_symbols_if_no_duplicates(&configs).unwrap();
    for config in configs.iter() {
        frb_codegen(config, &all_symbols).unwrap();
    }

    // Format the generated Dart code
    _ = std::process::Command::new("flutter")
        .arg("format")
        .arg("..")
        .spawn();
}
