use std::marker::PhantomData;

use stlcrts::*;
use stlcrts_macros::stlc;

fn main() {
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

    eval_to::<App<Lam<Arrow<Bool, Bool>, Var<I0>>, Lam<Bool, Var<I0>>>, Lam<Bool, Var<I0>>>();

    eval_to::<stlc! {(fn b: Bool => if b then false else true)(true)}, False>();

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

    type_checks::<
        stlc! {
            let not = fn b: Bool => if b then false else true in
            let and = fn a: Bool => fn b: Bool => if a then b else false in
            and true (not false)
        },
    >();

    let _res = eval::<stlc! { iszero 0 }>();

    eval_to::<
        stlc! {
            let add = fix (fn add: (Nat -> (Nat -> Nat)) => fn a: Nat => fn b: Nat =>
                if iszero a then b else succ (add (pred a) b)
            ) in add 3 15
        },
        stlc! { 18 },
    >();

    eval_to::<stlc! { pred 5 }, stlc! { 4 }>();

    let _res: std::marker::PhantomData<stlc! { 2 }> = eval::<
        stlc! { let id = fn f: (Nat -> Bool) => fn n: Nat => f n in
                    let iszerofn2 = id iszero in
                    succ (if iszerofn2 0 then 1 else 2)
        },
    >();
}
