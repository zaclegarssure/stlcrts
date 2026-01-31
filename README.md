# The Simply Typed Lambda Calculus in Rust Traits
This is a type-checker for the simply typed lambda calculus with booleans
and natural numbers written in Rust's trait.
I got the idea after discussing the Turing-completeness of Rust's type system with some colleagues and decided
to write some program in it such as a type-checker for the stlc.

## Usage
```rust
// Check that a term is well typed
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

// Check that a term evaluates to a given value
eval_to::<
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
    True,
>();

// Using fancy macro
eval_to::<
    stlc! {
        let rec add (a: Nat) (b: Nat) : Nat =
            if iszero a then b else succ (add (pred a) b)
        in
        let rec mul (a: Nat) (b: Nat) : Nat =
            if iszero a then 0 else add b (mul (pred a) b)
        in
        let rec fact (a: Nat) : Nat =
            if iszero a then 1 else mul a (fact (pred a))
        in
        fact 3
    },
    stlc! { 6 },
>();
```

## TODOs
- A Repl, by invoking `rustc` at runtime, because that's the most normal way to do it.
