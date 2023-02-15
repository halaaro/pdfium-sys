use std::{env, fs, io::ErrorKind, path::PathBuf};

pub(crate) fn gclient_build_dir() -> PathBuf {
    if let Ok(path) = env::var("PDFIUM_BUILD_DIR") {
        path.into()
    } else {
        let mut out_path: PathBuf = env::var("OUT_DIR").unwrap().into();
        out_path.push(".gclient");
        out_path
    }
}

pub(crate) fn pdfium_root_dir() -> PathBuf {
    let mut pdfium_root = gclient_build_dir();
    pdfium_root.push("pdfium");
    pdfium_root
}

pub(crate) fn pdfium_out_dir() -> PathBuf {
    let mut out_dir = pdfium_root_dir();
    out_dir.push("out");
    out_dir.push("Default");
    out_dir
}

pub(crate) fn pdfium_lib_dir() -> PathBuf {
    let mut lib_dir = pdfium_out_dir();
    lib_dir.push("obj");
    lib_dir
}

pub(crate) fn cache_dir() -> PathBuf {
    if let Ok(path) = env::var("PDFIUM_GCLIENT_CACHE") {
        path.into()
    } else {
        let mut cache_path = gclient_build_dir();
        cache_path.push("cache");
        cache_path
    }
}

pub(crate) fn repo_dir() -> PathBuf {
    // TODO: test that this is ok
    std::env::current_dir().unwrap()
}

pub(crate) fn src_dir() -> PathBuf {
    let mut src_dir = repo_dir();
    src_dir.push("pdfium");
    src_dir
}

pub(crate) fn mkdirs(dir: &PathBuf) {
    match fs::create_dir_all(dir) {
        Ok(_) => (),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => (),
        Err(_) => panic!("could not create path: {}", dir.display()),
    }
}
