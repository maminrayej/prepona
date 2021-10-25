//! The edge module contains structures and traits necessary to describe edges and anything related to them like [`EdgeToken`].

mod descriptors;
mod direction;
mod token;

pub use descriptors::*;
pub use direction::*;
pub use token::EdgeToken;
