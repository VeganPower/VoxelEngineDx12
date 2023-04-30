use std::path::PathBuf;
// use std::fs::File;
use std::process::{ Command, Output };
// use std::vec::Vec;
// use std::string::String;

// use windows::Win32::Graphics::Direct3D::*;
// use windows::core::{HSTRING, PCSTR};
// use windows::Win32::Graphics::Direct3D::*;
// use windows::Win32::Graphics::Direct3D::Fxc::*;

struct ShaderEntry
{
    shader_file : String,
    out_file : String,
    entry_point : String,
    profile : String
}

fn compile_shader(entry : &ShaderEntry, out_file : &str) -> std::io::Result<Output>
{
    return Command::new("dxc")
        .args(["-E", &entry.entry_point])
        .args(["-T", &entry.profile])
        .args(["-Fo", &out_file])
        .arg(&entry.shader_file)
        .output()
}

fn main() -> std::io::Result<()>
{
    let shaders : [ShaderEntry; 2] = [
        ShaderEntry {
            shader_file : String::from("shaders\\shaders.hlsl"),
            out_file    : String::from("vs.bin"),
            entry_point : String::from("VSMain"),
            profile     : String::from("vs_6_0"),
        },
        ShaderEntry {
            shader_file : String::from("shaders\\shaders.hlsl"),
            out_file    : String::from("ps.bin"),
            entry_point : String::from("PSMain"),
            profile     : String::from("ps_6_0"),
        },
    ];

    let out_dir = std::env::var("OUT_DIR").unwrap();
    for x in shaders {
        println!("cargo:rerun-if-changed={}", x.shader_file.as_str());

        let mut out_file = PathBuf::new();
        out_file.push(&out_dir);
        out_file.push(&x.out_file);

        let out_file_arg = out_file.as_os_str().to_str().unwrap();
        match compile_shader(&x, &out_file_arg) {
            Ok(_) => {
                let file_name = out_file.file_name().unwrap();
                let mut res_file = PathBuf::new();
                res_file.push(&out_dir);
                res_file.push("..\\..\\..\\..\\..\\resources");
                res_file.push(file_name);

                std::fs::copy(out_file_arg, res_file).expect("Copy");
            },
            Err(error_message) => {
                println!("{}", error_message);
                let custom_error = std::io::Error::new(std::io::ErrorKind::Other, "Shader compilation failed");
                return Err(custom_error);
            }
        };
    }
    return Ok(());
}