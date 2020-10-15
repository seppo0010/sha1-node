use wasm_bindgen::prelude::*;
use sha1::Sha1;
// /use js_sys::Promise;

/*#[wasm_bindgen]
extern "C" {

}*/

#[wasm_bindgen]
pub fn digest(string: &str) -> Option<String> {

    //let string = cx.argument::<JsString>(0)?.value();

    let mut m = Sha1::new();
    m.update(string.as_bytes());
    let dgst= m.digest().to_string();
    
    Some(dgst)

    
}
/*
register_module!(mut m, {
    m.export_function("digest", digest)?;
    Ok(())
});
*/
/*
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn digest(data: *mut c_char) -> *mut c_char {
    unsafe {
        let data = CStr::from_ptr(data);

        let mut m = Sha1::new();
        m.update(data.to_bytes());
        let dgst = m.digest().to_string();
        let s = CString::new(dgst).unwrap();
        s.into_raw()
    }
}
*/
