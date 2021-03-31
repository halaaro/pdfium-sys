// Copyright 2021 pdfium-sys Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate pdfium_sys as sys;

use std::env;
use std::ffi;
use std::os::raw::c_int;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!(
            "ERROR: unexpected number of arguments \n\nUSAGE:\n\t{} {}\n",
            args[0], "<input.pdf>"
        );
        std::process::exit(-1);
    }

    let file_path = &args[1];

    unsafe {
        //TODO: replace with FPDF_InitLibraryWithConfig
        sys::FPDF_InitLibrary();
    }
    println!("FPDF_InitLibrary called");

    let password = "";
    let doc = load_doc(file_path, password);
    if doc.is_null() {
        print_last_err();
        panic!("Problem loading document!");
    }
    println!(
        "FPDF_LoadDocument called with file_path=\"{}\", password=\"{}\"",
        file_path, password
    );

    let count: i32;
    unsafe {
        count = sys::FPDF_GetPageCount(doc) as i32;
    }
    println!("FPDF_GetPageCount returned {}", count);

    let mut width = 0.064;
    let mut height = 0.064;
    let mut index: c_int;

    for i in 0..count {
        let width_ptr: *mut f64 = &mut width;
        let height_ptr: *mut f64 = &mut height;
        index = i;
        let ret;
        unsafe {
            ret = sys::FPDF_GetPageSizeByIndex(doc, index, width_ptr, height_ptr);
        }
        if ret == 0 {
            print_last_err();
            panic!("Problem reading page size from document!");
        }

        println!(
            "FPDF_GetPageSizeByIndex with index = {} returned {}, width = {}, height = {}",
            index, ret, width, height
        );
    }

    unsafe {
        sys::FPDF_DestroyLibrary();
    }

    println!("FPDF_DestroyLibrary called");
}

fn load_doc(file_path: &str, password: &str) -> sys::FPDF_DOCUMENT {
    let c_file_path = ffi::CString::new(file_path).unwrap();
    let c_password = ffi::CString::new(password).unwrap();

    unsafe { sys::FPDF_LoadDocument(c_file_path.as_ptr(), c_password.as_ptr()) }
}

fn print_last_err() {
    unsafe {
        println!("FPDF_GetLastError returned {}", sys::FPDF_GetLastError());
    }
}
