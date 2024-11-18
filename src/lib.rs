use frida_gum::interceptor::Interceptor;
use frida_gum::{interceptor, Gum, Module, NativePointer};
use std::ffi::{CStr, CString};
use std::sync::OnceLock;
use {
    android_logger::Config,
    ctor::ctor,
    libc::{c_char, c_int, c_void},
    log::{info, LevelFilter},
};

#[ctor]
fn init() {
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag("rustNDK"),
    );
    info!("init success");
    // unsafe {
    //     hook_open();
    // }
}

type OpenFunc = unsafe extern "C" fn(*const c_char, flags: c_int) -> c_int;
static mut ORG_OPEN: Option<OpenFunc> = None;
unsafe extern "C" fn open_detour(name: *const c_char, flags: c_int) -> c_int {
    info!("hook open_detour: {}", CStr::from_ptr(name).to_str().unwrap());
    ORG_OPEN.unwrap()(name, flags)
}

 pub fn hook_open() {
    static CELL: OnceLock<Gum> = OnceLock::new();
    let gum = CELL.get_or_init(|| Gum::obtain());
    let module = Module::obtain(gum);
    let open = module.find_export_by_name(None, "open").unwrap();
    let mut interceptor = Interceptor::obtain(gum);

     unsafe {
         ORG_OPEN = Some(std::mem::transmute::<NativePointer,OpenFunc>(interceptor.replace(
             open,
             NativePointer(open_detour as *mut c_void),
             NativePointer(std::ptr::null_mut()),
         ).unwrap()
         ));
     }
}
