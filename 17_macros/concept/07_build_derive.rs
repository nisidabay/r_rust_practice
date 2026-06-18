// 07_build_derive.rs — Derive macros concept (document only)
//
// This file is a README-style document about derive macros.
// Derive macros (proc macros) are a different category from declarative macros
// (macro_rules!). They require the `proc-macro` crate type and external crates
// like `syn` and `quote` for parsing and generating token streams.
//
// This file does NOT compile as a proc-macro (that requires a separate crate).
// Instead, it documents the concepts and shows what a derive macro would look
// like as an external crate.
//
// To actually use derive macros, you'd create a separate crate with:
//   Cargo.toml: [lib] proc-macro = true
//   and depend on `syn`, `quote`, `proc-macro2`.

/*
// ============================================================
// DERIVE MACROS — Concepts
// ============================================================

What are they?
  Derive macros (aka "custom derive") use `#[derive(MyTrait)]` syntax.
  They are procedural macros that operate on the token stream of a struct,
  enum, or union, and generate additional trait implementations.

Key crates:
  - `syn`: Parses Rust source code into an AST
  - `quote`: Generates Rust token streams from interpolated values
  - `proc-macro2`: A wrapper around the compiler's proc_macro API

Lifecycle:
  1. User writes `#[derive(MyDerive)] struct Foo { ... }`
  2. Compiler invokes the proc-macro function in the derive crate
  3. The function receives a TokenStream representing the item
  4. It parses with syn, builds a new token stream with quote, returns it
  5. Compiler merges the generated code with the original item

Example derivation — HelloMacro:
  // In the proc-macro crate:
  #[proc_macro_derive(HelloMacro)]
  pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
      let ast: DeriveInput = syn::parse(input).unwrap();
      let name = &ast.ident;

      let gen = quote! {
          impl HelloMacro for #name {
              fn hello_macro() {
                  println!("Hello, Macro! My name is {}", stringify!(#name));
              }
          }
      };
      gen.into()
  }

  // In the user's code:
  #[derive(HelloMacro)]
  struct Pancakes;

  Pancakes::hello_macro(); // Prints: Hello, Macro! My name is Pancakes

The `DeriveInput` struct from syn contains:
  - ident: the name of the type
  - data: Data (enum, struct, union)
  - attrs: Vec<Attribute>
  - generics: Generics

For field-level processing:
  - Use syn::Data::Struct to access fields
  - Iterate over fields to generate per-field implementations

Attributes on derive macros:
  - #[proc_macro_derive(TraitName, attributes(my_attr))]
  - Lets the macro read helper attributes on fields

Attribute macros (#[my_attr]):
  - Similar to derive but placed on items directly
  - #[proc_macro_attribute]
  - Receive the attribute arguments AND the item token stream

Function-like proc macros:
  - #[proc_macro]
  - Like macro_rules! but with full Rust code processing power
  - Example: sql!(SELECT * FROM users WHERE id = 1)

When to use each:
  - macro_rules! : Simple code generation, pattern matching on tokens
  - Derive macros: When you need trait implementations on custom types
  - Attribute macros: When you need to wrap or transform entire items
  - Function-like proc macros: When you need full parsing power

Limitations of macro_rules! vs proc macros:
  Macro_rules!:
    + No external dependencies
    + Compiles with stable Rust
    + Simple to write for straightforward patterns
    - Cannot parse complex nested syntax
    - Limited to pattern matching (no arbitrary computation)
    - Harder to debug (expansion errors can be cryptic)

  Proc macros:
    + Full Rust computation during expansion
    + Can parse arbitrary syntax
    + Better error messages
    - Requires separate crate
    - Requires syn/quote dependencies
    - Slower compilation
*/

fn main() {
    println!("=== Derive Macros (documentation) ===");
    println!("");
    println!("Derive macros require a separate proc-macro crate.");
    println!("This file documents the concepts only.");
    println!("");
    println!("See a real proc-macro crate for compilable examples.");
    println!("Key crates: syn, quote, proc-macro2.");
    println!("");
    println!("macro_rules! is for pattern-based code generation.");
    println!("Derive/attribute macros are for AST-based code generation.");
}
