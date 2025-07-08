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

// T-Bool
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
