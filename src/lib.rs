#![cfg_attr(docsrs, feature(doc_cfg))]

mod common;
pub mod algo;
pub mod prop;
pub mod provide;
pub mod view;

#[cfg_attr(docsrs, doc(cfg(feature = "generate")))]
#[cfg(feature = "generate")]
pub mod gen;
