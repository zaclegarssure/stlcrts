# The Simply Typed Lambda Calculus in Rust Traits
This is a type-checker for the simply typed lambda calculus with booleans
and natural numbers written in Rust's trait.
I got the idea after discussing the Turing-completeness of Rust's type system with some colleagues and decided
to write some program in it such as a type-checker for the stlc.

## Usage
```rust
// For now the only thing that can be done is to type-check
// a program, which just checks that the given generic implements the
// WellTyped trait
type_checks::<
    // let not = \b: Bool -> if b then false else true in
    Let<
        Lam<Bool, If<Var<I0>, False, True>>,
        // let and = \a: Bool -> \b: Bool -> if a then b else false in
        Let<
            Lam<Bool, Lam<Bool, If<Var<ISucc<I0>>, Var<I0>, False>>>,
            // and true (not false)
            App<App<Var<I0>, True>, App<Var<ISucc<I0>>, False>>,
        >,
    >,
>();
```

## TODOs
- A proc-macro to write stlc code with a nice syntax. The example above would become:
  ```rust
  stlc! {
      let not = \b: Bool -> if b then false else true in
      let and = \a: Bool -> \b: Bool -> if a then b else false in
      and true (not false)
  }
  ```
- More terms such as `pred`, `is_zero`, `let_rec`.
- An interpreter written in traits
- A Repl, by invoking `rustc` at runtime, because yeah that's the most normal thing to do.
