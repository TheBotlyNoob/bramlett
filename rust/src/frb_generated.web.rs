// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.21.

// Section: imports

use super::*;
use crate::api::games::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::for_generated::wasm_bindgen;
use flutter_rust_bridge::for_generated::wasm_bindgen::prelude::*;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate_web!();

// Section: dart2rust

impl CstDecode<flutter_rust_bridge::for_generated::anyhow::Error> for String {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> flutter_rust_bridge::for_generated::anyhow::Error {
        unimplemented!()
    }
}
impl CstDecode<String> for String {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> String {
        self
    }
}
impl CstDecode<crate::api::games::Game>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::games::Game {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            8,
            "Expected 8 elements, got {}",
            self_.length()
        );
        crate::api::games::Game {
            name: self_.get(0).cst_decode(),
            exe: self_.get(1).cst_decode(),
            args: self_.get(2).cst_decode(),
            icon: self_.get(3).cst_decode(),
            url: self_.get(4).cst_decode(),
            uuid: self_.get(5).cst_decode(),
            sha256: self_.get(6).cst_decode(),
            state: self_.get(7).cst_decode(),
        }
    }
}
impl CstDecode<Vec<String>> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<String> {
        self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap()
            .iter()
            .map(CstDecode::cst_decode)
            .collect()
    }
}
impl CstDecode<Vec<crate::api::games::Game>>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<crate::api::games::Game> {
        self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap()
            .iter()
            .map(CstDecode::cst_decode)
            .collect()
    }
}
impl CstDecode<Vec<u8>> for Box<[u8]> {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<u8> {
        self.into_vec()
    }
}
impl CstDecode<flutter_rust_bridge::for_generated::anyhow::Error>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> flutter_rust_bridge::for_generated::anyhow::Error {
        unimplemented!()
    }
}
impl CstDecode<OpaqueBytes> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> OpaqueBytes {
        CstDecode::<
            RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<OpaqueBytes>>,
        >::cst_decode(self)
        .rust_auto_opaque_decode_owned()
    }
}
impl CstDecode<Progress> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Progress {
        CstDecode::<RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<Progress>>>::cst_decode(self).rust_auto_opaque_decode_owned()
    }
}
impl CstDecode<RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<OpaqueBytes>>>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(
        self,
    ) -> RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<OpaqueBytes>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }
        unsafe { decode_rust_opaque_nom((self.as_f64().unwrap() as usize) as _) }
    }
}
impl CstDecode<RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<Progress>>>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(
        self,
    ) -> RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<Progress>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }
        unsafe { decode_rust_opaque_nom((self.as_f64().unwrap() as usize) as _) }
    }
}
impl CstDecode<String> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl CstDecode<bool> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> bool {
        self.is_truthy()
    }
}
impl CstDecode<crate::api::games::GameState>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::games::GameState {
        (self.unchecked_into_f64() as i32).cst_decode()
    }
}
impl CstDecode<i32> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl CstDecode<Vec<u8>> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<u8> {
        self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Uint8Array>()
            .to_vec()
            .into()
    }
}
impl CstDecode<u64> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> u64 {
        ::std::convert::TryInto::try_into(
            self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::BigInt>()
                .unwrap(),
        )
        .unwrap()
    }
}
impl CstDecode<u8> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl CstDecode<usize> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> usize {
        self.unchecked_into_f64() as _
    }
}

#[wasm_bindgen]
pub fn wire_Progress_get_denominator(
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_get_denominator_impl(that)
}

#[wasm_bindgen]
pub fn wire_Progress_get_numerator(
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_get_numerator_impl(that)
}

#[wasm_bindgen]
pub fn wire_Progress_increment_denominator(
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_increment_denominator_impl(that)
}

#[wasm_bindgen]
pub fn wire_Progress_increment_numerator(
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_increment_numerator_impl(that)
}

#[wasm_bindgen]
pub fn wire_Progress_is_empty(
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_is_empty_impl(that)
}

#[wasm_bindgen]
pub fn wire_Progress_is_full(
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_is_full_impl(that)
}

#[wasm_bindgen]
pub fn wire_Progress_new() -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_new_impl()
}

#[wasm_bindgen]
pub fn wire_Progress_set_denominator(
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    denominator: u64,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_set_denominator_impl(that, denominator)
}

#[wasm_bindgen]
pub fn wire_Progress_set_numerator(
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    numerator: u64,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_Progress_set_numerator_impl(that, numerator)
}

#[wasm_bindgen]
pub fn wire_download_game(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    game: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    progress: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_download_game_impl(port_, game, progress)
}

#[wasm_bindgen]
pub fn wire_extract_zip(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    bytes: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    game: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    progress: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_extract_zip_impl(port_, bytes, game, progress)
}

#[wasm_bindgen]
pub fn wire_fetch_games(port_: flutter_rust_bridge::for_generated::MessagePort) {
    wire_fetch_games_impl(port_)
}

#[wasm_bindgen]
pub fn wire_init_app(port_: flutter_rust_bridge::for_generated::MessagePort) {
    wire_init_app_impl(port_)
}

#[wasm_bindgen]
pub fn wire_run_game(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    game: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_run_game_impl(port_, game)
}

#[wasm_bindgen]
pub fn rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockOpaqueBytes(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<OpaqueBytes>>::increment_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockOpaqueBytes(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<OpaqueBytes>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Progress>>::increment_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Progress>>::decrement_strong_count(ptr as _);
    }
}
