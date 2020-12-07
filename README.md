# is_variant

Generates methods to check variant

# Example

```rust
use is_variant_derive::IsVariant;
 
#[derive(IsVariant)]
enum TestEnum {
    A,
    B(),
    C(i32, i32),
    D { _name: String, _age: i32 },
}
 
fn main() {
    let x = TestEnum::C(1, 2);
    assert!(x.is_c());
 
    let x = TestEnum::A;
    assert!(x.is_a());
 
    let x = TestEnum::B();
    assert!(x.is_b());
 
    let x = TestEnum::D {_name: "Jane Doe".into(), _age: 30 };
    assert!(x.is_d());
}
```
