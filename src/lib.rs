use std::marker::PhantomData;

pub trait Term {}

pub struct True;
pub struct False;
impl Term for True {}
impl Term for False {}

pub struct Zero;
pub struct Succ<T: Term>(PhantomData<T>);
impl Term for Zero {}
impl<T: Term> Term for Succ<T> {}

// De Bruijn indices
pub trait Index {}
pub struct I0;
// I suck indeed
pub struct ISucc<I: Index>(PhantomData<I>);
impl Index for I0 {}
impl<I: Index> Index for ISucc<I> {}

// We use De Bruijn indices to represent variables
// to make it easier to index the environment in the trait impl constraints
pub struct Var<I: Index>(PhantomData<I>);
impl<I: Index> Term for Var<I> {}

pub struct Lam<Tp: Type, T: Term>(PhantomData<(Tp, T)>);
impl<Tp: Type, T: Term> Term for Lam<Tp, T> {}

pub struct If<Cond: Term, Then: Term, Else: Term>(PhantomData<(Cond, Then, Else)>);
impl<C: Term, T: Term, E: Term> Term for If<C, T, E> {}

pub struct App<F: Term, A: Term>(PhantomData<(F, A)>);
impl<F: Term, A: Term> Term for App<F, A> {}

/// Let-bindings, don't actualy bind things to a "name" since we use indices,
pub struct Let<T: Term, Body: Term>(PhantomData<(T, Body)>);
impl<T: Term, Body: Term> Term for Let<T, Body> {}

pub trait Type {}

pub struct Bool;
impl Type for Bool {}

pub struct Nat;
impl Type for Nat {}

pub struct Arrow<T: Type, U: Type>(PhantomData<(T, U)>);
impl<T: Type, U: Type> Type for Arrow<T, U> {}

// The environment for the typechecker, just a list of types
pub trait Env {}
pub struct EmptyEnv;
impl Env for EmptyEnv {}
pub struct TyCons<Tp: Type, Tl: Env>(PhantomData<(Tp, Tl)>);
impl<Tp: Type, Tl: Env> Env for TyCons<Tp, Tl> {}

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

// Evaluation

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
impl<T1: Term, V1: Value, T2: Term, V2: Value, C: Ctx> Eval<C> for Let<T1, T2>
where
    T1: Eval<C, Res = V1>,
    T2: Eval<CtxCons<V1, C>, Res = V2>,
{
    type Res = V2;
}

// For E-App, we first need to define a substitution function

// A term implements Subst<V, D, Res = R> if when substituting all occurences
// of variable D (it's a level because we use indices) we obtain R.
// Note that it also updates all references to variables beyound D.
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

pub fn eval_to<T, V>()
where
    V: Value,
    T: Term,
    T: Eval<EmptyCtx, Res = V>,
{
}

trait Select<T1: Term, T2: Term, C: Ctx> {
    type Res: Value;
}
impl<T1: Term, T2: Term, V1: Value, C: Ctx> Select<T1, T2, C> for True
where
    T1: Eval<C, Res = V1>,
{
    type Res = V1;
}
impl<T1: Term, T2: Term, V2: Value, C: Ctx> Select<T1, T2, C> for False
where
    T2: Eval<C, Res = V2>,
{
    type Res = V2;
}
impl<Cond: Term, V: Value, T1: Term, T2: Term, R: Value, C: Ctx> Eval<C> for If<Cond, T1, T2>
where
    Cond: Eval<C, Res = V>,
    V: Select<T1, T2, C, Res = R>,
{
    type Res = R;
}
