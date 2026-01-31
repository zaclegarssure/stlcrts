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
            let not = fn b: Bool -> if b then false else true in
            let and = fn a: Bool -> fn b: Bool -> if a then b else false in
            and true (not false)
        },
    >();

    eval_to::<stlc! { 5 }, stlc! { 5 }>();

    let _res: std::marker::PhantomData<Succ<Succ<Succ<Succ<Zero>>>>> =
        eval::<stlc! { if true then 4 else 3 }>();
}
