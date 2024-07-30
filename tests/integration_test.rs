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

#[from(A)]
#[derive(Debug, PartialEq, Eq)]
enum TransformExtraSameName {
      #[mapping]
      Foo,
      #[mapping]
      Bar,
      #[allow(unused)]
      Baz
}

#[from(A)]
#[derive(Debug, PartialEq, Eq)]
enum TransformExtraDifferentName {
      #[mapping(A::Foo)]
      V1,
      #[mapping(A::Bar)]
      V2,
      #[allow(unused)]
      V3
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

#[test]
fn testTransformExtraSameName() {
      assert_eq!(TransformExtraSameName::from(A::Foo), TransformExtraSameName::Foo);
      assert_eq!(TransformExtraSameName::from(A::Bar), TransformExtraSameName::Bar);
}

#[test]
fn testTransformExtraDifferentName() {
      assert_eq!(TransformExtraDifferentName::from(A::Foo), TransformExtraDifferentName::V1);
      assert_eq!(TransformExtraDifferentName::from(A::Bar), TransformExtraDifferentName::V2);
}

