use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_read(port_: MessagePort, path: String) {
    wire_read_impl(port_, path)
}

#[wasm_bindgen]
pub fn wire_write(port_: MessagePort, path: String, data: JsValue) {
    wire_write_impl(port_, path, data)
}

// Section: allocate functions

#[wasm_bindgen]
pub fn new_box_autoadd_i32_0(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[wasm_bindgen]
pub fn new_box_autoadd_u32_0(value: u32) -> *mut u32 {
    support::new_leak_box_ptr(value)
}

// Section: impl Wire2Api

impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}

impl Wire2Api<Option<String>> for Option<String> {
    fn wire2api(self) -> Option<String> {
        self.map(Wire2Api::wire2api)
    }
}

impl Wire2Api<Option<Vec<u8>>> for Option<Box<[u8]>> {
    fn wire2api(self) -> Option<Vec<u8>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Tag> for JsValue {
    fn wire2api(self) -> Tag {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            7,
            "Expected 7 elements, got {}",
            self_.length()
        );
        Tag {
            title: self_.get(0).wire2api(),
            artist: self_.get(1).wire2api(),
            album: self_.get(2).wire2api(),
            year: self_.get(3).wire2api(),
            genre: self_.get(4).wire2api(),
            duration: self_.get(5).wire2api(),
            picture: self_.get(6).wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}
// Section: impl Wire2Api for JsValue

impl Wire2Api<String> for JsValue {
    fn wire2api(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Option<String>> for JsValue {
    fn wire2api(self) -> Option<String> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<i32>> for JsValue {
    fn wire2api(self) -> Option<i32> {
        (self != 0).then(|| *Wire2Api::<Box<i32>>::wire2api(self))
    }
}
impl Wire2Api<Option<u32>> for JsValue {
    fn wire2api(self) -> Option<u32> {
        (self != 0).then(|| *Wire2Api::<Box<u32>>::wire2api(self))
    }
}
impl Wire2Api<Option<Vec<u8>>> for JsValue {
    fn wire2api(self) -> Option<Vec<u8>> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<u32> for JsValue {
    fn wire2api(self) -> u32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u8> for JsValue {
    fn wire2api(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<u8>> for JsValue {
    fn wire2api(self) -> Vec<u8> {
        self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
    }
}
