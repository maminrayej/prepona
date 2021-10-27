//! Contains notations that are used across the crate to describe [time complexity] of methods and algorithms.
//!
//! * `func_name`: Time complexity of calling the function *func_name*.
//!     * For example consider this code snippet:
//!     ```rust
//!         fn A() { 
//!             // -- snip 
//!         }
//!         
//!         fn B() { 
//!             A()
//!         }
//!     ```
//!     To describe the time complexity of `B` we use the notation: O(`A`).
//!     Whenever possible, instead of just putting the name of the function, we put a link to function in the documentation.
//!     Another example would be this code snippet:
//!     ```rust
//!         fn A() { 
//!             // -- snip
//!         }
//!
//!         fn C() {
//!             // -- snip
//!         }
//!
//!         fn B() { 
//!             A(); 
//!             C(); 
//!         }
//!     ```
//!     In this scenario we use O(`A` + `C`) to describe the time complexity of `B`.
//! * `func_name on entity`: Time complexity of calling function *func_name* with *entity* as its argument.
//!     * For example consider code snippet:
//!     ```rust
//!         struct A;
//!         
//!         fn C(a: &A) { 
//!             // -- snip
//!         }
//!         
//!         fn B() { 
//!             C(&A); 
//!         }
//!     ```
//!     To describe time complexity of `B` we use the notation: O(`C` on `A`).
//!     We usually use this to refer to methods that we can not link to directly.
//!     For example consider code snippet below (It's is the real reason why we need this notation):
//!     ```rust
//!         trait UIndex: std::ops::Index<usize> {}
//!
//!         fn B<T: UIndex>(value: T) { 
//!             let v = &value[0]; 
//!         }
//!     ```
//!     Function `B` can be called with any type that implements `UIndex` as its argument.
//!     So to describe the time complexity of function `B` we use the notation O(`Index::index on UIndex`).
//!     Meaning the operation determining the time complexity of `B` is calling the `index` function on the type that implements `UIndex`.
//!     Because we can not use O(`UIndex::index`), we use this notation instead.
//! * `|V<sub>src</sub>|`: Number of sources.
//! * `|V<sub>dst</sub>|`: Number of destinations.
//!
//! [time complexity]: https://en.wikipedia.org/wiki/Time_complexity
