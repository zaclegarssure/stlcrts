use crate::term::*;

// A term T implements WellTyped<E> if it is well-typed under environment E.
// Furthermore its type is Tp
pub trait WellTyped<E: Env> {
    type Tp: Type;
}

// T-True
impl<E: Env> WellTyped<E> for True {
    type Tp = Bool;
}

// T-False
impl<E: Env> WellTyped<E> for False {
    type Tp = Bool;
}

// T-Zero
impl<E: Env> WellTyped<E> for Zero {
    type Tp = Nat;
}

// T-Succ
impl<E: Env, T: Term> WellTyped<E> for Succ<T>
where
    T: WellTyped<E, Tp = Nat>,
{
    type Tp = Nat;
}

// T-IsZero
impl<E: Env, T: Term> WellTyped<E> for IsZero<T>
where
    T: WellTyped<E, Tp = Nat>,
{
    type Tp = Bool;
}

// T-Pred
impl<E: Env, T: Term> WellTyped<E> for Pred<T>
where
    T: WellTyped<E, Tp = Nat>,
{
    type Tp = Bool;
}

// T-VarSucc
impl<N: Index, E: Env, Tp: Type> WellTyped<TyCons<Tp, E>> for Var<ISucc<N>>
where
    Var<N>: WellTyped<E>,
{
    type Tp = <Var<N> as WellTyped<E>>::Tp;
}

// T-Var0
impl<E: Env, Tp: Type> WellTyped<TyCons<Tp, E>> for Var<I0> {
    type Tp = Tp;
}

// T-Lambda
impl<E: Env, Tp: Type, T: Term> WellTyped<E> for Lam<Tp, T>
where
    T: WellTyped<TyCons<Tp, E>>,
{
    type Tp = Arrow<Tp, <T as WellTyped<TyCons<Tp, E>>>::Tp>;
}

// T-App
impl<E: Env, ITp: Type, OTp: Type, T: Term, F: Term> WellTyped<E> for App<F, T>
where
    T: WellTyped<E, Tp = ITp>,
    F: WellTyped<E, Tp = Arrow<ITp, OTp>>,
{
    type Tp = OTp;
}

// T-If
impl<E: Env, Tp: Type, Cond: Term, Then: Term, Else: Term> WellTyped<E> for If<Cond, Then, Else>
where
    Cond: WellTyped<E, Tp = Bool>,
    Then: WellTyped<E, Tp = Tp>,
    Else: WellTyped<E, Tp = Tp>,
{
    type Tp = Tp;
}

// T-Let
impl<E: Env, Tp: Type, T: Term, Body: Term> WellTyped<E> for Let<T, Body>
where
    Body: WellTyped<TyCons<Tp, E>>,
    T: WellTyped<E, Tp = Tp>,
{
    type Tp = <Body as WellTyped<TyCons<Tp, E>>>::Tp;
}

/// Util function to type-check a given term (given as a generic argument)
pub fn type_checks<T>()
where
    T: WellTyped<EmptyEnv>,
    T: Term,
{
}
