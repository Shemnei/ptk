#![allow(dead_code)]
//#![allow(rustdoc::private_intra_doc_links)]
#![deny(
	deprecated_in_future,
	exported_private_dependencies,
	future_incompatible,
	missing_copy_implementations,
	// TODO: rustdoc::missing_crate_level_docs,
	rustdoc::broken_intra_doc_links,
	// TODO: missing_docs,
	// TODO: clippy::missing_docs_in_private_items,
	missing_debug_implementations,
	private_in_public,
	rust_2021_compatibility,
	rust_2021_prelude_collisions,
	rust_2021_incompatible_or_patterns,
	rust_2021_prefixes_incompatible_syntax,
    rust_2021_incompatible_closure_captures,
	trivial_casts,
	trivial_numeric_casts,
	unsafe_code,
	unstable_features,
	unused_import_braces,
	unused_qualifications,

	// clippy attributes
	clippy::missing_const_for_fn,
	clippy::redundant_pub_crate,
	clippy::use_self
)]
#![cfg_attr(docsrs, feature(doc_cfg), feature(doc_alias))]

pub mod loc;
pub mod pos;
pub mod span;
pub mod src;
