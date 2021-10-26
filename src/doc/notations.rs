//! Contains notations that are used across the crate to describe [time complexity] of methods and algorithms.
//!
//! * `func_name`: Time complexity of calling the function *func_name*.
//!     * For example consider this code snippet:
//!     ```rust compile_fail
//!         fn A() { ... }
//!
//!         fn B() { A() }
//!     ```
//!     To describe the time complexity of `B` we use the notation: O(`A`).
//!     Whenever possible, instead of just putting the name of the function, we put a link to function in the documentation.\
//!     Another example would be this code snippet:
//!     ```rust compile_fail
//!         fn A() { ... }
//! 
//!         fn C() { ... }
//! 
//!         fn B() { A(); C(); }
//!     ```
//!     In this scenario we use O(`A` + `C`) to describe the time complexity of `B`.
//! * `|V<sub>src</sub>|`: Number of sources.
//! * `|V<sub>dst</sub>|`: Number of destinations.
//!
//! [time complexity]: https://en.wikipedia.org/wiki/Time_complexity
