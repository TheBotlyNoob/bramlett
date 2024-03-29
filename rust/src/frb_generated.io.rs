// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.21.

// Section: imports

use super::*;
use crate::api::games::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate_io!();

// Section: dart2rust

impl CstDecode<flutter_rust_bridge::for_generated::anyhow::Error>
    for *mut wire_cst_list_prim_u_8_strict
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> flutter_rust_bridge::for_generated::anyhow::Error {
        unimplemented!()
    }
}
impl CstDecode<OpaqueBytes> for usize {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> OpaqueBytes {
        CstDecode::<
            RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<OpaqueBytes>>,
        >::cst_decode(self)
        .rust_auto_opaque_decode_owned()
    }
}
impl CstDecode<Progress> for usize {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Progress {
        CstDecode::<RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<Progress>>>::cst_decode(self).rust_auto_opaque_decode_owned()
    }
}
impl CstDecode<RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<OpaqueBytes>>>
    for usize
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(
        self,
    ) -> RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<OpaqueBytes>> {
        unsafe { decode_rust_opaque_nom(self as _) }
    }
}
impl CstDecode<RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<Progress>>>
    for usize
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(
        self,
    ) -> RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<Progress>> {
        unsafe { decode_rust_opaque_nom(self as _) }
    }
}
impl CstDecode<String> for *mut wire_cst_list_prim_u_8_strict {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> String {
        let vec: Vec<u8> = self.cst_decode();
        String::from_utf8(vec).unwrap()
    }
}
impl CstDecode<crate::api::games::Game> for *mut wire_cst_game {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::games::Game {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::api::games::Game>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::api::games::Game> for wire_cst_game {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::games::Game {
        crate::api::games::Game {
            name: self.name.cst_decode(),
            exe: self.exe.cst_decode(),
            args: self.args.cst_decode(),
            icon: self.icon.cst_decode(),
            url: self.url.cst_decode(),
            uuid: self.uuid.cst_decode(),
            sha256: self.sha256.cst_decode(),
            state: self.state.cst_decode(),
        }
    }
}
impl CstDecode<Vec<String>> for *mut wire_cst_list_String {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<String> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(CstDecode::cst_decode).collect()
    }
}
impl CstDecode<Vec<crate::api::games::Game>> for *mut wire_cst_list_game {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<crate::api::games::Game> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(CstDecode::cst_decode).collect()
    }
}
impl CstDecode<Vec<u8>> for *mut wire_cst_list_prim_u_8_strict {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<u8> {
        unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl NewWithNullPtr for wire_cst_game {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
            exe: core::ptr::null_mut(),
            args: core::ptr::null_mut(),
            icon: core::ptr::null_mut(),
            url: core::ptr::null_mut(),
            uuid: core::ptr::null_mut(),
            sha256: core::ptr::null_mut(),
            state: Default::default(),
        }
    }
}
impl Default for wire_cst_game {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_wire_Progress_get_denominator(
    that: usize,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_get_denominator_impl(that)
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_wire_Progress_get_numerator(
    that: usize,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_get_numerator_impl(that)
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_wire_Progress_increment_denominator(
    that: usize,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_increment_denominator_impl(that)
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_wire_Progress_increment_numerator(
    that: usize,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_increment_numerator_impl(that)
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_wire_Progress_is_empty(
    that: usize,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_is_empty_impl(that)
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_wire_Progress_is_full(
    that: usize,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_is_full_impl(that)
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_wire_Progress_new(
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_new_impl()
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_wire_Progress_set_denominator(
    that: usize,
    denominator: u64,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_set_denominator_impl(that, denominator)
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_wire_Progress_set_numerator(
    that: usize,
    numerator: u64,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_set_numerator_impl(that, numerator)
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_wire_download_game(
    port_: i64,
    game: *mut wire_cst_game,
    progress: usize,
) {
    wire_download_game_impl(port_, game, progress)
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_wire_extract_zip(
    port_: i64,
    bytes: usize,
    game: *mut wire_cst_game,
    progress: usize,
) {
    wire_extract_zip_impl(port_, bytes, game, progress)
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_wire_fetch_games(port_: i64) {
    wire_fetch_games_impl(port_)
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_wire_init_app(port_: i64) {
    wire_init_app_impl(port_)
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_wire_run_game(port_: i64, game: *mut wire_cst_game) {
    wire_run_game_impl(port_, game)
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockOpaqueBytes(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<OpaqueBytes>>::increment_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockOpaqueBytes(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<OpaqueBytes>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Progress>>::increment_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Progress>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_cst_new_box_autoadd_game() -> *mut wire_cst_game {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(wire_cst_game::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_cst_new_list_String(
    len: i32,
) -> *mut wire_cst_list_String {
    let wrap = wire_cst_list_String {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(
            <*mut wire_cst_list_prim_u_8_strict>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_cst_new_list_game(len: i32) -> *mut wire_cst_list_game {
    let wrap = wire_cst_list_game {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(
            <wire_cst_game>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn frbgen_bramletts_games_cst_new_list_prim_u_8_strict(
    len: i32,
) -> *mut wire_cst_list_prim_u_8_strict {
    let ans = wire_cst_list_prim_u_8_strict {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(ans)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_game {
    name: *mut wire_cst_list_prim_u_8_strict,
    exe: *mut wire_cst_list_prim_u_8_strict,
    args: *mut wire_cst_list_String,
    icon: *mut wire_cst_list_prim_u_8_strict,
    url: *mut wire_cst_list_prim_u_8_strict,
    uuid: *mut wire_cst_list_prim_u_8_strict,
    sha256: *mut wire_cst_list_prim_u_8_strict,
    state: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_String {
    ptr: *mut *mut wire_cst_list_prim_u_8_strict,
    len: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_game {
    ptr: *mut wire_cst_game,
    len: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_prim_u_8_strict {
    ptr: *mut u8,
    len: i32,
}
