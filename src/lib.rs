// Copyright (C) 2019-2022 The apca Developers
// SPDX-License-Identifier: GPL-3.0-or-later

#![type_length_limit = "536870912"]
#![allow(clippy::let_unit_value, clippy::unreadable_literal)]
#![warn(
  bad_style,
  dead_code,
  future_incompatible,
  illegal_floating_point_literal_pattern,
  improper_ctypes,
  late_bound_lifetime_arguments,
  missing_copy_implementations,
  missing_debug_implementations,
  missing_docs,
  no_mangle_generic_items,
  non_shorthand_field_patterns,
  nonstandard_style,
  overflowing_literals,
  path_statements,
  patterns_in_fns_without_body,
  private_in_public,
  proc_macro_derive_resolution_fallback,
  renamed_and_removed_lints,
  rust_2018_compatibility,
  rust_2018_idioms,
  stable_features,
  trivial_bounds,
  trivial_numeric_casts,
  type_alias_bounds,
  tyvar_behind_raw_pointer,
  unconditional_recursion,
  unreachable_code,
  unreachable_patterns,
  unreachable_pub,
  unstable_features,
  unstable_name_collisions,
  unused,
  unused_comparisons,
  unused_import_braces,
  unused_lifetimes,
  unused_qualifications,
  unused_results,
  where_clauses_object_safety,
  while_true,
  rustdoc::broken_intra_doc_links
)]

//! A crate for interacting with the Alpaca API.

// Do not allow `rustls` and `native-tls` to be used at the same time.
#[cfg(all(feature = "rustls", feature = "native-tls"))]
compile_error!("features `rustls` and `native-tls` are mutually exclusive");

#[macro_use]
extern crate http_endpoint;

#[macro_use]
mod endpoint;

/// A module comprising the functionality backing interactions with the
/// trading API.
pub mod api;

/// A module for retrieving market data.
pub mod data;

mod api_info;
mod client;
mod error;
mod subscribable;
mod util;
mod websocket;

use std::borrow::Cow;

pub use crate::api_info::ApiInfo;
pub use crate::client::Client;
pub use crate::endpoint::ApiError;
pub use crate::error::Error;
pub use crate::error::RequestError;
pub use crate::subscribable::Subscribable;

type Str = Cow<'static, str>;
