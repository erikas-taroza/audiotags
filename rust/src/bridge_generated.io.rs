use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_read(port_: i64, path: *mut wire_uint_8_list) {
    wire_read_impl(port_, path)
}

#[no_mangle]
pub extern "C" fn wire_write(port_: i64, path: *mut wire_uint_8_list, data: *mut wire_Tag) {
    wire_write_impl(port_, path, data)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_tag_0() -> *mut wire_Tag {
    support::new_leak_box_ptr(wire_Tag::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u32_0(value: u32) -> *mut u32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_list_picture_0(len: i32) -> *mut wire_list_picture {
    let wrap = wire_list_picture {
        ptr: support::new_leak_vec_ptr(<wire_Picture>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<Tag> for *mut wire_Tag {
    fn wire2api(self) -> Tag {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Tag>::wire2api(*wrap).into()
    }
}
impl Wire2Api<u32> for *mut u32 {
    fn wire2api(self) -> u32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}

impl Wire2Api<Vec<Picture>> for *mut wire_list_picture {
    fn wire2api(self) -> Vec<Picture> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<Picture> for wire_Picture {
    fn wire2api(self) -> Picture {
        Picture {
            picture_type: self.picture_type.wire2api(),
            mime_type: self.mime_type.wire2api(),
            bytes: self.bytes.wire2api(),
        }
    }
}

impl Wire2Api<Tag> for wire_Tag {
    fn wire2api(self) -> Tag {
        Tag {
            title: self.title.wire2api(),
            artist: self.artist.wire2api(),
            album: self.album.wire2api(),
            year: self.year.wire2api(),
            genre: self.genre.wire2api(),
            duration: self.duration.wire2api(),
            pictures: self.pictures.wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_picture {
    ptr: *mut wire_Picture,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Picture {
    picture_type: i32,
    mime_type: i32,
    bytes: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Tag {
    title: *mut wire_uint_8_list,
    artist: *mut wire_uint_8_list,
    album: *mut wire_uint_8_list,
    year: *mut u32,
    genre: *mut wire_uint_8_list,
    duration: *mut u32,
    pictures: *mut wire_list_picture,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_Picture {
    fn new_with_null_ptr() -> Self {
        Self {
            picture_type: Default::default(),
            mime_type: Default::default(),
            bytes: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_Tag {
    fn new_with_null_ptr() -> Self {
        Self {
            title: core::ptr::null_mut(),
            artist: core::ptr::null_mut(),
            album: core::ptr::null_mut(),
            year: core::ptr::null_mut(),
            genre: core::ptr::null_mut(),
            duration: core::ptr::null_mut(),
            pictures: core::ptr::null_mut(),
        }
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
