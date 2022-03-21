fn main() {
    #[cfg(target_os = "windows")]
    {
        extern crate winres;

        let mut res = winres::WindowsResource::new();
        res.set_icon("osmon.ico");
        res.set_language(winapi::um::winnt::MAKELANGID(
            winapi::um::winnt::LANG_UZBEK,
            winapi::um::winnt::SUBLANG_UZBEK_LATIN,
        ));
        res.compile().unwrap();
    }
}
