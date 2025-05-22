# kerml-rust

A Rust library for representing and manipulating **Kerml** models using statically generated, trait-based structures that simulate object-oriented paradigms in a Rust-idiomatic way.

## Overview

Kerml is a model specification language with an object-oriented architecture, including classes, inheritance, attributes, and method overloading. Rust, however, is not inherently object-oriented, which poses a challenge for faithful Kerml integration.

This crate provides a statically generated Rust implementation of the Kerml specification using advanced trait-based encoding to model:

- Class hierarchies and inheritance
- Attribute access across superclasses
- Method overloading
- Upcasting and downcasting

The library has been generated from an ecore specification of Kerml, using a custom code generator built specifically for this purpose.



## Crate Structure

The crate is composed of four main modules:

- `root`: Corresponds to the root section of the Kerml specification
- `core`: Models the core elements
- `kernel`: Encodes the kernel-level types and behaviors
- `generated`: Automatically generated code from the Kerml Ecore spec

Each class in Kerml maps to a dedicated Rust module containing:

- A struct representing the class and its attributes
- Traits for accessing attributes (including inherited ones)
- Enums for modeling subclass relationships
- Upcast/downcast traits and implementations
- Traits for methods, including overloading support



## Inheritance Encoding

For each class, we encode inheritance and method overloading as follows:

- **Structs** represent attributes and store instances of superclasses.
- **Enums** model subclass polymorphism with variants like `Itself` or subclass enums.
- **Traits** provide:
  - Attribute access (local and inherited)
  - Upcasting and downcasting
  - Default and overloaded method handling via a `DescendantOf<T>` pattern

This design enables:

- Statically typed method dispatch with overload support
- Encapsulation of inheritance trees
- Compile-time guarantees on method implementation completeness




## Example (Class: `Import`)

The `Import` class:

- Inherits from `Relationship`
- Has two subclasses: `MembershipImport` and `NamespaceImport`

Rust representation includes:

- A struct `Import` with:
  - Fields for its own attributes
  - A `relationship_inst` field
- A trait for each superclass (`Relationship`) to access inherited attributes
- An enum `ImportEnum` with:
  - `Itself(Import)`
  - Variants for each subclass enum
- Upcast and downcast traits
- Method traits with default and overload-aware resolution



## Generated Traits

For each class:

- Traits are defined for each method (inherited and local)
- The programmer must implement the methods declared for the current class
- Traits support overloads via a generic `DescendantOf<OriginClass>` pattern with an associated `Via` type



## Disclaimer

This crate is a **prototype** and is not yet production-ready.