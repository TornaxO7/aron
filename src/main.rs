// aron (c) Nikolas Wipper 2020

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

extern crate custom_derive;
extern crate enum_derive;

use crate::assembler::{Module, ObjectFileType};
use crate::parse::parser::parse_lines;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

mod assembler;
mod instructions;
mod number;
mod parse;

fn main() {
    let args = std::env::args();

    for arg in args.skip(1) {
        let path = Path::new(&arg);
        if path.extension().unwrap() == OsStr::new("o") {
            eprintln!("Skipping {}, has .o extension", arg);
            continue;
        }

        let mut file = File::open(&path).unwrap();

        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();
        let parsed_lines = parse_lines(arg.clone(), code);

        if let Ok(parsed_lines) = parsed_lines {
            let module = Module::from_lines(parsed_lines);

            let out_name = path.with_extension("o");

            module.write_to_file(out_name, ObjectFileType::MachO).expect("Couldn't write module");
        } else {
            exit(1);
        }
    }
}
