#![cfg_attr(docsrs, feature(doc_cfg))]

mod misc;
#[macro_use]
mod cfgs;

pub mod algo;
pub mod give;
pub mod prop;
pub mod view;

#[cfg_attr(docsrs, doc(cfg(feature = "generate")))]
#[cfg(feature = "generate")]
pub mod make;
