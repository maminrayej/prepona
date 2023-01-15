#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod algo;
mod common;
pub mod prop;
pub mod provide;
pub mod view;

#[cfg_attr(docsrs, doc(cfg(feature = "generate")))]
#[cfg(feature = "generate")]
pub mod gen;
