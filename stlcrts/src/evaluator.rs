use crate::term::*;
use std::marker::PhantomData;

pub trait Value: Term {}
impl Value for True {}
impl Value for False {}
impl Value for Zero {}
impl<N: Term> Value for Succ<N> where N: Value {}
impl<Tp: Type, T: Term> Value for Lam<Tp, T> {}

pub trait Ctx {}
pub struct EmptyCtx;
impl Ctx for EmptyCtx {}
pub struct CtxCons<V: Value, Tl: Ctx>(PhantomData<(V, Tl)>);
impl<V: Value, Tl: Ctx> Ctx for CtxCons<V, Tl> {}

pub trait Eval<C: Ctx> {
    type Res: Value;
}

// E-True
impl<C: Ctx> Eval<C> for True {
    type Res = True;
}

// E-False
impl<C: Ctx> Eval<C> for False {
    type Res = False;
}

// E-Zero
impl<C: Ctx> Eval<C> for Zero {
    type Res = Zero;
}

// E-Succ
impl<T: Term, V: Value, C: Ctx> Eval<C> for Succ<T>
where
    T: Eval<C, Res = V>,
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

impl<T: Term, C: Ctx, V: Value, R: Value> Eval<C> for IsZero<T>
where
    T: Eval<C, Res = V>,
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
impl<T: Term, C: Ctx, V: Value, V2: Value> Eval<C> for Pred<T>
where
    T: Eval<C, Res = V>,
    V: PredResult<Res = V2>,
{
    type Res = V2;
}

// E-VarSucc
impl<N: Index, C: Ctx, V: Value, V2: Value> Eval<CtxCons<V, C>> for Var<ISucc<N>>
where
    Var<N>: Eval<C, Res = V2>,
{
    type Res = V2;
}

// E-Var0
impl<C: Ctx, V: Value> Eval<CtxCons<V, C>> for Var<I0> {
    type Res = V;
}

// E-Lam
impl<Tp: Type, T: Term, C: Ctx> Eval<C> for Lam<Tp, T> {
    type Res = Lam<Tp, T>;
}

// E-Let
impl<T1: Term, V1: Value, T2: Term, V2: Value, C: Ctx> Eval<C> for Let<T1, T2>
where
    T1: Eval<C, Res = V1>,
    T2: Eval<CtxCons<V1, C>, Res = V2>,
{
    type Res = V2;
}

// For E-App, we first need to define a substitution function

// A term implements Subst<V, D, Res = R> if when substituting all occurrences
// of variable D (it's a level because we use indices) we obtain R.
// Note that it also updates all references to variables beyond D.
//
// More precisely it implements the following pseudocode:
//
// let substitute term value depth =
//     match term with
//         | Var i ->
//              if i = depth then value
//              else if i > depth then Var (i - 1)
//              else Var i
//         | Lam body -> Lam (substitute body value (depth + 1))
//         | App t1 t2 -> App (substitute t1 value depth) (substitute t2 value depth)
//         | ... (the other cases are trivial)
trait Subst<V: Value, D: Index> {
    type Res: Term;
}

// subst True -> True
impl<V: Value, D: Index> Subst<V, D> for True {
    type Res = True;
}

// subst False -> False
impl<V: Value, D: Index> Subst<V, D> for False {
    type Res = False;
}

impl<V: Value, D: Index> Subst<V, D> for Zero {
    type Res = Zero;
}

impl<V: Value, D: Index, T: Term, ST: Term> Subst<V, D> for Succ<T>
where
    T: Subst<V, D, Res = ST>,
{
    type Res = Succ<ST>;
}

// subst if c then th else el -> if subst c then subst th else subst el
impl<V: Value, D: Index, Cond: Term, SCond: Term, Then: Term, SThen: Term, Else: Term, SElse: Term>
    Subst<V, D> for If<Cond, Then, Else>
where
    Cond: Subst<V, D, Res = SCond>,
    Then: Subst<V, D, Res = SThen>,
    Else: Subst<V, D, Res = SElse>,
{
    type Res = If<SCond, SThen, SElse>;
}

//  subst Lam body -> Lam (subst body (depth + 1))
impl<V: Value, D: Index, Body: Term, Tp: Type, ST: Term> Subst<V, D> for Lam<Tp, Body>
where
    Body: Subst<V, ISucc<D>, Res = ST>,
{
    type Res = Lam<Tp, ST>;
}

impl<V: Value, D: Index, F: Term, SF: Term, A: Term, SA: Term> Subst<V, D> for App<F, A>
where
    F: Subst<V, D, Res = SF>,
    A: Subst<V, D, Res = SA>,
{
    type Res = App<SF, SA>;
}

impl<V: Value, D: Index, T: Term, ST: Term> Subst<V, D> for IsZero<T>
where
    T: Subst<V, D, Res = ST>,
{
    type Res = IsZero<ST>;
}

impl<V: Value, D: Index, T: Term, ST: Term> Subst<V, D> for Pred<T>
where
    T: Subst<V, D, Res = ST>,
{
    type Res = Pred<ST>;
}

// It's a bit of a mess, will comment later
trait VarSubst<V: Value, Original: Term, Updated: Term> {
    type Res: Term;
}
impl<V: Value, Original: Term, Updated: Term> VarSubst<V, Original, Updated> for (I0, I0) {
    type Res = V;
}
impl<V: Value, N: Index, O: Term, U: Term> VarSubst<V, O, U> for (ISucc<N>, I0) {
    type Res = U;
}
impl<V: Value, N: Index, O: Term, U: Term> VarSubst<V, O, U> for (I0, ISucc<N>) {
    type Res = O;
}
impl<V: Value, N1: Index, N2: Index, V2: Term, O: Term, U: Term> VarSubst<V, O, U>
    for (ISucc<N1>, ISucc<N2>)
where
    (N1, N2): VarSubst<V, O, U, Res = V2>,
{
    type Res = V2;
}

trait IPred {
    type Res: Index;
}
impl IPred for I0 {
    type Res = I0;
}
impl<N: Index> IPred for ISucc<N> {
    type Res = N;
}

impl<V: Value, D: Index, N: Index, N2: Index, V2: Term> Subst<V, D> for Var<N>
where
    (N, D): VarSubst<V, Var<N>, Var<N2>, Res = V2>,
    N: IPred<Res = N2>,
{
    type Res = V2;
}

// Now we can finally implement E-App
impl<C: Ctx, F: Term, A: Term, T: Term, ST: Term, V: Value, V2: Value, Tp: Type> Eval<C>
    for App<F, A>
where
    F: Eval<C, Res = Lam<Tp, T>>,
    T: Subst<V, I0, Res = ST>,
    A: Eval<C, Res = V>,
    ST: Eval<C, Res = V2>,
{
    type Res = V2;
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
impl<Cond: Term, V: Value, T1: Term, T2: Term, T3: Term, R: Value, C: Ctx> Eval<C>
    for If<Cond, T1, T2>
where
    Cond: Eval<C, Res = V>,
    V: Select<T1, T2, Res = T3>,
    T3: Eval<C, Res = R>,
{
    type Res = R;
}

pub fn eval_to<T, V>()
where
    V: Value,
    T: Term,
    T: Eval<EmptyCtx, Res = V>,
{
}

pub fn eval<T: Eval<EmptyCtx>>() -> PhantomData<T::Res> {
    PhantomData
}
