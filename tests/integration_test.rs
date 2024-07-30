#![allow(non_snake_case)]

use enum_from::from;

enum A {
      Foo,
      Bar
}

#[from(A)]
#[derive(Debug, PartialEq, Eq)]
enum TransformBySameName {
      Foo,
      Bar
}

#[from(A)]
#[derive(Debug, PartialEq, Eq)]
enum TransformCustom {
      #[mapping(A::Foo)]
      V1,
      #[mapping(A::Bar)]
      V2
}

#[test]
fn testTransformSameName() {
      assert_eq!(TransformBySameName::from(A::Foo), TransformBySameName::Foo);
      assert_eq!(TransformBySameName::from(A::Bar), TransformBySameName::Bar);
}

#[test]
fn testTransformDifferentName() {
      assert_eq!(TransformCustom::from(A::Foo), TransformCustom::V1);
      assert_eq!(TransformCustom::from(A::Bar), TransformCustom::V2);
}
