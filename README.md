# enum-from
Procedural macro to generate mapping from an enum to another

# Usage
Annotate the enum with `enum_from::from`.
`std::convert::From<FromEnum>` will be implemented for the annotated enum.
```rust
use enum_from::from

enum FromEnum {
      Foo,
      Bar
}

#[from(FromEnum)]
enum ToEnum {
      Foo,
      Bar
}
```

If the variant in the from enum and target enum has the same name, no extra annotation is needed.
Otherwise, add `#[mapping(FromEnum::Variant)]` to each variant with different name.

```rust
#[from(FromEnum)]
enum DifferentName {
      #[mapping(FromEnum::Foo)]
      A,
      Bar // no need to specify name for variant with same name 
}
```

If the target enum has more variants than the from enum, each variant in the target enum has to be annotated with `#[mapping]`.
```rust
#[from(FromEnum)]
enum MoreVariants {
      #[mapping(FromEnum::Foo)]
      A,
      #[mapping]
      Bar, // no need to specify name for variant with same name
      Unrelated
}
```

# To be implemented
* Allow a fallback variant. Example:
```rust
enum A {
      Foo,
      Bar,
      Baz,
}

#[from(FromEnum)]
enum Fallback {
      Foo,
      #[fallback]
      Fallback // Bar.into() and Baz.into() will return Fallback
}
```

* Allow a variant to match multiple incoming variants
```rust
#[from(FromEnum)]
enum A {
      #[mapping(FromEnum::Foo)]
      #[mapping(FromEnum::Bar)]
      Multiple
}
```

