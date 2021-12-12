// #![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]
// #![deny(rustdoc::broken_intra_doc_links)]
// #![warn(rustdoc::missing_crate_level_docs)]

//! Prepona is a highly flexible and parameterized graph data-structure and algorithm library.
//!
//! # Preconditions and postconditions
//! Every method in the library describes a set of preconditions and postconditions.
//! When a method is called, assuming its preconditions are met, it must produce the output or side effects that satisfies the postconditions.
//! But if the preconditions of a method is not satisfied, the method is allowed to `panic` or put the struct in an invalid state.
//!
//! Therefore every method in the library also has a **checked** version. Checked version of each method ensures its preconditions are met before proceeding.
//! If preconditions are not satisfied, the checked version of the method will return an error.
//!
//! ### **Why**
//! Checking for validity of inputs has two downsides:
//! - **It's slow**: Checking for validity of inputs on every function call is slow. Especially in data structures where a function can be called many times consecutively.
//!              In many scenarios, for examples in algorithms, the inputs are provided by an iterator which always returns valid data. Checking the validity of these inputs is just a waste of resources.
//!              Also, many of the preconditions can be met implicitly by the user. Doubling checking everything just makes things slow unnecessarily.
//! - **It complicates the basic implementation and reduces readability**: Adding boundary and existential checking to the basic implementation makes the code more verbose and
//!              reduces readability of sometimes already complex code.
//!
//! ### Conclusion
//! Decoupling the basic implementation from its input validation, makes the base code simpler, more readable and more maintainable. But it exposes the programmer to logical errors.
//! On the other hand, validating inputs on every function call has its own downsides(as mentioned above). To have best of both worlds and to not impose any design choice on the users of the library,
//! for every method there is an equivalent **checked** version that does the checking before proceeding.
//!
//! **As a rule of thumb** \
//! If you can satisfy the preconditions implicitly and want maximum performance, use the basic implementation.\
//! If you are working with unknown inputs and you're not sure about their validity, use the checked version of the method.

// TODO: Implement generators

pub mod algo;
pub mod common;
pub mod doc;
pub mod gen;
pub mod graph;
pub mod provide;
pub mod storage;

#[cfg(test)]
pub mod test_utils;
