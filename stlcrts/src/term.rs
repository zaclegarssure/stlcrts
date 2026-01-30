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
pub struct ISucc<I: Index>(PhantomData<I>);
impl Index for I0 {}
impl<I: Index> Index for ISucc<I> {}

// We use De Bruijn indices to represent variables
pub struct Var<I: Index>(PhantomData<I>);
impl<I: Index> Term for Var<I> {}

pub struct Lam<Tp: Type, T: Term>(PhantomData<(Tp, T)>);
impl<Tp: Type, T: Term> Term for Lam<Tp, T> {}

pub struct If<Cond: Term, Then: Term, Else: Term>(PhantomData<(Cond, Then, Else)>);
impl<C: Term, T: Term, E: Term> Term for If<C, T, E> {}

pub struct App<F: Term, A: Term>(PhantomData<(F, A)>);
impl<F: Term, A: Term> Term for App<F, A> {}

/// Let-bindings, don't actually bind things to a "name" since we use indices,
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
