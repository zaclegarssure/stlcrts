use stlcrts::*;

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
}
