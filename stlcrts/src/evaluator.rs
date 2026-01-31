use crate::{
    evaluator::{shiftby1::ShiftBy1, shiftbyneg1::ShiftByNeg1, subst::Subst},
    term::*,
};
use std::marker::PhantomData;

pub trait Value: Term {}
impl Value for True {}
impl Value for False {}
impl Value for Zero {}
impl<N: Term> Value for Succ<N> where N: Value {}
impl<Tp: Type, T: Term> Value for Lam<Tp, T> {}

mod shiftby1 {
    use super::*;

    pub trait ShiftBy1 {
        type Res: Term;
    }
    trait Shift<C: Index> {
        type Res: Term;
    }
    impl<T: Term, R: Term> ShiftBy1 for T
    where
        T: Shift<I0, Res = R>,
    {
        type Res = R;
    }

    impl<C: Index> Shift<C> for True {
        type Res = True;
    }
    impl<C: Index> Shift<C> for False {
        type Res = False;
    }

    impl<C: Index> Shift<C> for Zero {
        type Res = Zero;
    }
    impl<C: Index, T: Term, Tprime: Term> Shift<C> for Succ<T>
    where
        T: Shift<C, Res = Tprime>,
    {
        type Res = Succ<Tprime>;
    }
    impl<C: Index, T: Term, Tprime: Term> Shift<C> for Pred<T>
    where
        T: Shift<C, Res = Tprime>,
    {
        type Res = Pred<Tprime>;
    }
    impl<C: Index, T: Term, Tprime: Term> Shift<C> for IsZero<T>
    where
        T: Shift<C, Res = Tprime>,
    {
        type Res = IsZero<Tprime>;
    }

    trait ShiftVar<C: Index> {
        type Res: Index;
    }
    impl<K: Index> ShiftVar<I0> for K {
        type Res = ISucc<K>;
    }
    impl<N: Index> ShiftVar<ISucc<N>> for I0 {
        type Res = I0;
    }
    impl<N: Index, K: Index, R: Index> ShiftVar<ISucc<N>> for ISucc<K>
    where
        K: ShiftVar<N, Res = R>,
    {
        type Res = ISucc<R>;
    }

    impl<C: Index, I: Index, R: Index> Shift<C> for Var<I>
    where
        I: ShiftVar<C, Res = R>,
    {
        type Res = Var<R>;
    }

    impl<C: Index, Tp: Type, T: Term, R: Term> Shift<C> for Lam<Tp, T>
    where
        T: Shift<ISucc<C>, Res = R>,
    {
        type Res = Lam<Tp, R>;
    }

    impl<C: Index, T1: Term, T1prime: Term, T2: Term, T2prime: Term> Shift<C> for App<T1, T2>
    where
        T1: Shift<C, Res = T1prime>,
        T2: Shift<C, Res = T2prime>,
    {
        type Res = App<T1prime, T2prime>;
    }

    impl<C: Index, T1: Term, T1prime: Term, T2: Term, T2prime: Term, T3: Term, T3prime: Term>
        Shift<C> for If<T1, T2, T3>
    where
        T1: Shift<C, Res = T1prime>,
        T2: Shift<C, Res = T2prime>,
        T3: Shift<C, Res = T3prime>,
    {
        type Res = If<T1prime, T2prime, T3prime>;
    }

    impl<C: Index, T1: Term, T1prime: Term, T2: Term, T2prime: Term> Shift<C> for Let<T1, T2>
    where
        T1: Shift<C, Res = T1prime>,
        T2: Shift<ISucc<C>, Res = T2prime>,
    {
        type Res = Let<T1prime, T2prime>;
    }

    impl<C: Index, T: Term, Tprime: Term> Shift<C> for Fix<T>
    where
        T: Shift<C, Res = Tprime>,
    {
        type Res = Fix<Tprime>;
    }
}

mod subst {

    use crate::evaluator::shiftby1::ShiftBy1;

    use super::*;

    pub trait Subst<J: Index, S: Term> {
        type Res: Term;
    }
    impl<J: Index, S: Term> Subst<J, S> for True {
        type Res = True;
    }
    impl<J: Index, S: Term> Subst<J, S> for False {
        type Res = False;
    }

    impl<J: Index, S: Term> Subst<J, S> for Zero {
        type Res = Zero;
    }
    impl<J: Index, S: Term, T: Term, Tprime: Term> Subst<J, S> for Succ<T>
    where
        T: Subst<J, S, Res = Tprime>,
    {
        type Res = Succ<Tprime>;
    }
    impl<J: Index, S: Term, T: Term, Tprime: Term> Subst<J, S> for Pred<T>
    where
        T: Subst<J, S, Res = Tprime>,
    {
        type Res = Pred<Tprime>;
    }
    impl<J: Index, S: Term, T: Term, Tprime: Term> Subst<J, S> for IsZero<T>
    where
        T: Subst<J, S, Res = Tprime>,
    {
        type Res = IsZero<Tprime>;
    }

    trait Select<S: Term, K: Term> {
        type Res: Term;
    }
    impl<S: Term, K: Term> Select<S, K> for (I0, I0) {
        type Res = S;
    }
    impl<S: Term, K: Term, N: Index> Select<S, K> for (ISucc<N>, I0) {
        type Res = K;
    }
    impl<S: Term, K: Term, N: Index> Select<S, K> for (I0, ISucc<N>) {
        type Res = K;
    }
    impl<S: Term, K: Term, N1: Index, N2: Index, R: Term> Select<S, K> for (ISucc<N1>, ISucc<N2>)
    where
        (N1, N2): Select<S, K, Res = R>,
    {
        type Res = R;
    }

    impl<J: Index, S: Term, I: Index, R: Term> Subst<J, S> for Var<I>
    where
        (I, J): Select<S, Var<I>, Res = R>,
    {
        type Res = R;
    }

    impl<J: Index, S: Term, Tp: Type, T: Term, Sprime: Term, Tprime: Term> Subst<J, S> for Lam<Tp, T>
    where
        S: ShiftBy1<Res = Sprime>,
        T: Subst<ISucc<J>, Sprime, Res = Tprime>,
    {
        type Res = Lam<Tp, Tprime>;
    }

    impl<J: Index, S: Term, T1: Term, T1prime: Term, T2: Term, T2prime: Term> Subst<J, S>
        for App<T1, T2>
    where
        T1: Subst<J, S, Res = T1prime>,
        T2: Subst<J, S, Res = T2prime>,
    {
        type Res = App<T1prime, T2prime>;
    }

    impl<
        J: Index,
        S: Term,
        T1: Term,
        T1prime: Term,
        T2: Term,
        T2prime: Term,
        T3: Term,
        T3prime: Term,
    > Subst<J, S> for If<T1, T2, T3>
    where
        T1: Subst<J, S, Res = T1prime>,
        T2: Subst<J, S, Res = T2prime>,
        T3: Subst<J, S, Res = T3prime>,
    {
        type Res = If<T1prime, T2prime, T3prime>;
    }

    impl<J: Index, S: Term, Sprime: Term, T1: Term, T1prime: Term, T2: Term, T2prime: Term>
        Subst<J, S> for Let<T1, T2>
    where
        T1: Subst<J, S, Res = T1prime>,
        S: ShiftBy1<Res = Sprime>,
        T2: Subst<ISucc<J>, Sprime, Res = T2prime>,
    {
        type Res = Let<T1prime, T2prime>;
    }

    impl<J: Index, S: Term, T: Term, Tprime: Term> Subst<J, S> for Fix<T>
    where
        T: Subst<J, S, Res = Tprime>,
    {
        type Res = Fix<Tprime>;
    }
}

mod shiftbyneg1 {
    use super::*;

    pub trait ShiftByNeg1 {
        type Res: Term;
    }
    trait Shift<C: Index> {
        type Res: Term;
    }
    impl<T: Term, R: Term> ShiftByNeg1 for T
    where
        T: Shift<I0, Res = R>,
    {
        type Res = R;
    }

    impl<C: Index> Shift<C> for True {
        type Res = True;
    }
    impl<C: Index> Shift<C> for False {
        type Res = False;
    }
    impl<C: Index> Shift<C> for Zero {
        type Res = Zero;
    }
    impl<C: Index, T: Term, Tprime: Term> Shift<C> for Succ<T>
    where
        T: Shift<C, Res = Tprime>,
    {
        type Res = Succ<Tprime>;
    }
    impl<C: Index, T: Term, Tprime: Term> Shift<C> for Pred<T>
    where
        T: Shift<C, Res = Tprime>,
    {
        type Res = Pred<Tprime>;
    }
    impl<C: Index, T: Term, Tprime: Term> Shift<C> for IsZero<T>
    where
        T: Shift<C, Res = Tprime>,
    {
        type Res = IsZero<Tprime>;
    }

    trait ShiftVar<C: Index> {
        type Res: Index;
    }
    impl<K: Index> ShiftVar<I0> for ISucc<K> {
        type Res = K;
    }
    impl<N: Index> ShiftVar<ISucc<N>> for I0 {
        type Res = I0;
    }
    impl<N: Index, K: Index, R: Index> ShiftVar<ISucc<N>> for ISucc<K>
    where
        K: ShiftVar<N, Res = R>,
    {
        type Res = ISucc<R>;
    }

    impl<C: Index, I: Index, R: Index> Shift<C> for Var<I>
    where
        I: ShiftVar<C, Res = R>,
    {
        type Res = Var<R>;
    }

    impl<C: Index, Tp: Type, T: Term, R: Term> Shift<C> for Lam<Tp, T>
    where
        T: Shift<ISucc<C>, Res = R>,
    {
        type Res = Lam<Tp, R>;
    }

    impl<C: Index, T1: Term, T1prime: Term, T2: Term, T2prime: Term> Shift<C> for App<T1, T2>
    where
        T1: Shift<C, Res = T1prime>,
        T2: Shift<C, Res = T2prime>,
    {
        type Res = App<T1prime, T2prime>;
    }

    impl<C: Index, T1: Term, T1prime: Term, T2: Term, T2prime: Term, T3: Term, T3prime: Term>
        Shift<C> for If<T1, T2, T3>
    where
        T1: Shift<C, Res = T1prime>,
        T2: Shift<C, Res = T2prime>,
        T3: Shift<C, Res = T3prime>,
    {
        type Res = If<T1prime, T2prime, T3prime>;
    }

    impl<C: Index, T1: Term, T1prime: Term, T2: Term, T2prime: Term> Shift<C> for Let<T1, T2>
    where
        T1: Shift<C, Res = T1prime>,
        T2: Shift<ISucc<C>, Res = T2prime>,
    {
        type Res = Let<T1prime, T2prime>;
    }

    impl<C: Index, T: Term, Tprime: Term> Shift<C> for Fix<T>
    where
        T: Shift<C, Res = Tprime>,
    {
        type Res = Fix<Tprime>;
    }
}

pub trait Eval {
    type Res: Value;
}

// E-True
impl Eval for True {
    type Res = True;
}

// E-False
impl Eval for False {
    type Res = False;
}

// E-Lam
impl<Tp: Type, T: Term> Eval for Lam<Tp, T> {
    type Res = Lam<Tp, T>;
}

// E-App
impl<
    T1: Term,
    T2: Term,
    Tp: Type,
    T1prime: Term,
    V2: Value,
    V2prime: Term,
    T1primeprime: Term,
    R: Term,
    Rprime: Value,
> Eval for App<T1, T2>
where
    T1: Eval<Res = Lam<Tp, T1prime>>,
    T2: Eval<Res = V2>,
    V2: ShiftBy1<Res = V2prime>,
    T1prime: Subst<I0, V2prime, Res = T1primeprime>,
    T1primeprime: ShiftByNeg1<Res = R>,
    R: Eval<Res = Rprime>,
{
    type Res = Rprime;
}

// E-Let
impl<T1: Term, V1: Value, T2: Term, V1prime: Term, T2prime: Term, T2primeprime: Term, Rprime: Value>
    Eval for Let<T1, T2>
where
    T1: Eval<Res = V1>,
    V1: ShiftBy1<Res = V1prime>,
    T2: Subst<I0, V1prime, Res = T2prime>,
    T2prime: ShiftByNeg1<Res = T2primeprime>,
    T2primeprime: Eval<Res = Rprime>,
{
    type Res = Rprime;
}

// E-Fix
impl<T: Term, Tp: Type, T1: Term, R: Value> Eval for Fix<T>
where
    T: Eval<Res = Lam<Tp, T1>>,
    T1: Subst<I0, Fix<Lam<Tp, T1>>, Res = R>,
{
    type Res = R;
}

// For E-If we need to use a little trick,
// because if we implement two rules E-True and E-False
// then rustc will complain that there might be overallping
// instances for Eval. Therefore we use a helper trait Select
// wich given two terms selects the approriate one. For True
// it selects the first and for False it selects the second.
trait Select<T1: Term, T2: Term> {
    type Res: Term;
}
impl<T1: Term, T2: Term> Select<T1, T2> for True {
    type Res = T1;
}
impl<T1: Term, T2: Term> Select<T1, T2> for False {
    type Res = T2;
}
impl<Cond: Term, V: Value, T1: Term, T2: Term, T3: Term, R: Value> Eval for If<Cond, T1, T2>
where
    Cond: Eval<Res = V>,
    V: Select<T1, T2, Res = T3>,
    T3: Eval<Res = R>,
{
    type Res = R;
}

// // E-Zero
impl Eval for Zero {
    type Res = Zero;
}

// E-Succ
impl<T: Term, V: Value> Eval for Succ<T>
where
    T: Eval<Res = V>,
{
    type Res = Succ<V>;
}

trait IsZeroResult {
    type Res: Value;
}
impl IsZeroResult for Zero {
    type Res = True;
}
impl<T: Term> IsZeroResult for Succ<T> {
    type Res = False;
}

impl<T: Term, V: Value, R: Value> Eval for IsZero<T>
where
    T: Eval<Res = V>,
    V: IsZeroResult<Res = R>,
{
    type Res = R;
}

trait PredResult {
    type Res: Value;
}
impl PredResult for Zero {
    type Res = Zero;
}
impl<V: Value> PredResult for Succ<V> {
    type Res = V;
}
impl<T: Term, V: Value, V2: Value> Eval for Pred<T>
where
    T: Eval<Res = V>,
    V: PredResult<Res = V2>,
{
    type Res = V2;
}

pub fn eval_to<T, V>()
where
    V: Value,
    T: Term,
    T: Eval<Res = V>,
{
}

pub fn eval<T: Eval>() -> PhantomData<T::Res> {
    PhantomData
}
