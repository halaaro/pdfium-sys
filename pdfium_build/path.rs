use std::{env, fs, io::ErrorKind, path::PathBuf};

pub fn gclient_build_dir() -> PathBuf {
    if let Ok(path) = env::var("PDFIUM_GCLIENT_BUILD") {
        path.into()
    } else {
        let mut gclient_path: PathBuf = target_dir().parent().unwrap().into();
        gclient_path.push("gclient");
        gclient_path
    }
}

fn target_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .into()
}

pub fn pdfium_root_dir() -> PathBuf {
    let mut pdfium_root = gclient_build_dir();
    pdfium_root.push("pdfium");
    pdfium_root
}

pub fn pdfium_out_dir() -> PathBuf {
    let mut out_dir = target_dir();
    out_dir.push("gn_out");
    out_dir
}

pub fn pdfium_lib_dir() -> PathBuf {
    let mut lib_dir = pdfium_out_dir();
    lib_dir.push("obj");
    lib_dir
}

pub fn cache_dir() -> PathBuf {
    if let Ok(path) = env::var("PDFIUM_GCLIENT_CACHE") {
        path.into()
    } else {
        let mut cache_path = gclient_build_dir();
        cache_path.push("cache");
        cache_path
    }
}

pub fn repo_dir() -> PathBuf {
    // TODO: test that this is ok
    std::env::current_dir().unwrap()
}

pub fn src_dir() -> PathBuf {
    let mut src_dir = repo_dir();
    src_dir.push("pdfium");
    src_dir
}

pub fn mkdirs(dir: &PathBuf) {
    match fs::create_dir_all(dir) {
        Ok(_) => (),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => (),
        Err(_) => panic!("could not create path: {}", dir.display()),
    }
}
