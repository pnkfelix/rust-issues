/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![feature(box_syntax)]
#![feature(custom_attribute)]
#![feature(custom_derive)]
#![feature(plugin)]
#![feature(proc_macro)]

extern crate hyper;
extern crate hyper_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use hyper::header::{Headers};
use hyper::http::RawStatus;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomResponse {
    #[serde(deserialize_with = "::hyper_serde::deserialize",
            serialize_with = "::hyper_serde::serialize")]
    pub headers: Headers,
    #[serde(deserialize_with = "::hyper_serde::deserialize",
            serialize_with = "::hyper_serde::serialize")]
    pub raw_status: RawStatus,
    pub body: Vec<u8>
}
