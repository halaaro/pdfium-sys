use pdfium_sys as sys;

fn main() {
    unsafe {
        sys::FPDF_InitLibrary();
        sys::FPDF_DestroyLibrary();
    }
}
