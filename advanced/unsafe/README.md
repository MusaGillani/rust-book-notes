# Dereferencing a Raw pointer

Unsafe Rust has two new types similar to references called _raw pointers_ 

Immutable -> `*const T` 
    > can't be directly assigned to after being dereferenced.
Mutable -> `*mut T`


Different from references and smart pointers, raw pointers:

- Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
- Aren’t guaranteed to point to valid memory
- Are allowed to be null
- Don’t implement any automatic cleanup

