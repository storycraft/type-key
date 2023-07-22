# TypeKey
Unique type identifier for any non-static type (unlike `core::any::Any`)

## Usage
```rust
use type_key::TypeKey;

let a = 1;
let closure = || &a;

let key = TypeKey::of_val(&closure);
```

## Implementation detail
The `TypeKey` wraps `TypeId` of closure in `TypeKey::of`. Since the closure captures its environment, the returned `TypeId` is unique for each type.

For more explanation about closure lifetime, see: https://github.com/rust-lang/rust/issues/27086

## License
MIT
