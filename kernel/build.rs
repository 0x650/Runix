/*
 * Copyright 2023 Runix Project Contributors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::env;

use std::fs;

use std::error::Error;
use std::ffi::OsString;
use std::fs::DirEntry;
use std::path::Path;

// Assembly related things mostly taken from Aero
// (I hate Rust build system with my entire being)
fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut inc_files = vec![];

    visit_dirs(Path::new("src"), &mut |entry| {
        let path = entry.path();

        match path.extension() {
            Some(ext) if ext.eq(&OsString::from("inc")) => {
                let path = path
                    .to_str()
                    .expect("Invalid UTF-8 for file path");
                inc_files.push(path.to_string())
            }

            _ => (),
        }
    })?;

    inc_files = inc_files
        .iter()
        .map(|e| {
            let e = e.split('/').collect::<Vec<_>>();
            e[..e.len() - 1].join("/")
        })
        .collect::<Vec<_>>();

    visit_dirs(Path::new("src"), &mut |entry: &DirEntry| {
        let path = entry.path();

        let object_os = path.file_name().expect("Failed to get file name");
        let object_file = object_os.to_str().expect("Invalid UTF-8 for file name");

        match path.extension() {
            Some(ext) if ext.eq(&OsString::from("asm")) => {
                let mut build = nasm_rs::Build::new();

                build
                    .file(&path)
                    .flag("-felf64")
                    .target("x86_64-unknown-none");

                println!("{:?}", inc_files);

                for include in &inc_files {
                    build.include(include);
                }

                build
                    .compile(object_file)
                    .expect("Failed to compile assembly");

                println!("cargo:rustc-link-lib=static={}", object_file);
            }

            _ => (),
        }
    })?;

    let cwd_path = env::current_dir().expect("Can't get current directory");
    let cwd = cwd_path.display();
    println!("cargo:rerun-if-changed={cwd}/build.rs");
    println!("cargo:rerun-if-changed={cwd}/Cargo.toml");
    println!("cargo:rerun-if-changed={cwd}/Cargo.lock");
    println!("cargo:rerun-if-changed={cwd}/src");
    println!("cargo:rerun-if-changed={cwd}/linker.ld");
    // Tell cargo to pass the linker script to the linker...
    println!("cargo:rustc-link-arg=-Tlinker.ld");
    // ...and to re-run if it changes.
    println!("cargo:rerun-if-changed=linker.ld");

    Ok(())
}
