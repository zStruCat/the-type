# the-type
A crate providing function which helps specifying type within the expression.
## Motivation
Haskell allows type annotation for expression:
```haskell
snoc (xs::[a])(x::a) = xs++[x] :: [a]
```
For Idris, the same function can be achieved by 'the' function
```idris
the : (ty : Type) -> ty -> ty
the ty x = x
```

There're also a RFC[^1] and ongoing implementation[^2] similar to the forementioned in Rust. This crate provide the function in library.
[^1]: https://github.com/rust-lang/rfcs/pull/803 
[^2]: https://github.com/rust-lang/rust/issues/23416

## Usage

```rust
use the_type::the;
let foo: u8 = 0;

// typical method
let items1 : [usize; 4] = [foo.into(); 4];

// won't compile becuase Into is a generic trait but self.into() is not a generic method
// let items2 = [foo.into::<usize>(); 4];

// currently an experimental feature:
// let items3 = [foo.into(): usize; 4];

// an alternative way: 
let items4 = [usize::from(foo); 4];

// a more intuitive way:
let items5 = [the::<usize>(foo.into()); 4];
```