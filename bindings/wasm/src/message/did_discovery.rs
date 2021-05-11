// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::comm;
use wasm_bindgen::prelude::*;

use crate::wasm_did::WasmDID;
use crate::wasm_url::WasmUrl;
use crate::wasm_uuid::WasmUuid;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct DidRequest(pub(crate) comm::DidRequest);

impl_wasm_accessors!(DidRequest, {
  context => String,
  thread => WasmUuid,
  callback_url => WasmUrl,
  response_requested => Option<bool>,
  id => Option<WasmDID>,
});

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct DidResponse(pub(crate) comm::DidResponse);

impl_wasm_accessors!(DidResponse, {
  context => String,
  thread => WasmUuid,
  id => WasmDID,
  callback_url => Option<WasmUrl>,
  response_requested => Option<bool>,
});
