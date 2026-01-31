use syn::{
    Ident, LitBool, LitInt, Result, Token,
    parse::{Parse, ParseStream},
};

#[derive(Clone)]
enum Tp {
    Bool,
    Nat,
    Arrow(Box<Tp>, Box<Tp>),
}

impl Parse for Tp {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(kw::Bool) {
            input.parse::<kw::Bool>()?;
            Ok(Tp::Bool)
        } else if input.peek(kw::Nat) {
            input.parse::<kw::Nat>()?;
            Ok(Tp::Nat)
        } else if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let left = content.parse()?;
            content.parse::<Token![->]>()?;
            let right = content.parse()?;
            Ok(Tp::Arrow(Box::new(left), Box::new(right)))
        } else {
            Err(syn::Error::new(
                input.span(),
                "expected type (Bool, Nat, or (Tp -> Tp))",
            ))
        }
    }
}

/// AST Obtained from the macro, which uses string as names rather than indices
enum Expr {
    Bool(bool),
    Nat(u64),
    Var(String),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Lam {
        param: String,
        tp: Tp,
        body: Box<Expr>,
    },
    App(Box<Expr>, Box<Expr>),
    Let {
        name: String,
        value: Box<Expr>,
        body: Box<Expr>,
    },
    IsZero(Box<Expr>),
    Pred(Box<Expr>),
}

mod kw {
    syn::custom_keyword!(then);
    syn::custom_keyword!(Bool);
    syn::custom_keyword!(Nat);
    syn::custom_keyword!(iszero);
    syn::custom_keyword!(pred);
}

impl Parse for Expr {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![let]) {
            input.parse::<Token![let]>()?;
            let name: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let value = input.parse()?;
            input.parse::<Token![in]>()?;
            let body = input.parse()?;
            Ok(Expr::Let {
                name: name.to_string(),
                value: Box::new(value),
                body: Box::new(body),
            })
        } else if input.peek(Token![if]) {
            input.parse::<Token![if]>()?;
            let cond = input.parse()?;
            input.parse::<kw::then>()?;
            let then_branch = input.parse()?;
            input.parse::<Token![else]>()?;
            let else_branch = input.parse()?;
            Ok(Expr::If(
                Box::new(cond),
                Box::new(then_branch),
                Box::new(else_branch),
            ))
        } else if input.peek(Token![fn]) {
            input.parse::<Token![fn]>()?;
            let param: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let tp: Tp = input.parse()?;
            input.parse::<Token![=>]>()?;
            let body = input.parse()?;
            Ok(Expr::Lam {
                param: param.to_string(),
                tp,
                body: Box::new(body),
            })
        } else {
            parse_application(input)
        }
    }
}

fn parse_application(input: ParseStream) -> Result<Expr> {
    let mut expr = parse_atom(input)?;

    while !input.is_empty()
        && !input.peek(Token![in])
        && !input.peek(kw::then)
        && !input.peek(Token![else])
    {
        let arg = parse_atom(input)?;
        expr = Expr::App(Box::new(expr), Box::new(arg));
    }

    Ok(expr)
}

fn parse_atom(input: ParseStream) -> Result<Expr> {
    if input.peek(LitBool) {
        let b: LitBool = input.parse()?;
        Ok(Expr::Bool(b.value))
    } else if input.peek(syn::token::Paren) {
        let content;
        syn::parenthesized!(content in input);
        content.parse()
    } else if input.peek(LitInt) {
        let n: LitInt = input.parse()?;
        let value = n.base10_parse::<u64>()?;
        Ok(Expr::Nat(value))
    } else if input.peek(kw::iszero) {
        input.parse::<kw::iszero>()?;
        // Little trick, but to make it possible for users to treat iszero
        // as a regular function we eta-expand it at parse time
        Ok(Expr::Lam {
            param: "x".to_string(),
            tp: Tp::Nat,
            body: Box::new(Expr::IsZero(Box::new(Expr::Var("x".to_string())))),
        })
    } else if input.peek(kw::pred) {
        input.parse::<kw::pred>()?;
        // Same as above
        Ok(Expr::Lam {
            param: "x".to_string(),
            tp: Tp::Nat,
            body: Box::new(Expr::Pred(Box::new(Expr::Var("x".to_string())))),
        })
    } else {
        let ident: Ident = input.parse()?;
        Ok(Expr::Var(ident.to_string()))
    }
}

/// AST but where names have been replaced by de Bruijn indices.
/// It could be generated directly at parse time but doing
/// it in two steps is a bit simpler.
enum DBExpr {
    True,
    False,
    Nat(u64),
    Var(usize),
    If(Box<DBExpr>, Box<DBExpr>, Box<DBExpr>),
    Lam(Tp, Box<DBExpr>),
    App(Box<DBExpr>, Box<DBExpr>),
    Let(Box<DBExpr>, Box<DBExpr>),
    IsZero(Box<DBExpr>),
    Pred(Box<DBExpr>),
}

/// Lower an Expr into a DBExpr, in the environment
fn lower(expr: &Expr, env: &mut Vec<String>) -> DBExpr {
    match expr {
        Expr::Bool(true) => DBExpr::True,
        Expr::Bool(false) => DBExpr::False,
        Expr::Nat(n) => DBExpr::Nat(*n),

        Expr::Var(name) => {
            let idx = env
                .iter()
                .rev()
                .position(|n| n == name)
                .expect("unbound variable");
            DBExpr::Var(idx)
        }

        Expr::If(c, t, e) => DBExpr::If(
            Box::new(lower(c, env)),
            Box::new(lower(t, env)),
            Box::new(lower(e, env)),
        ),

        Expr::Lam { param, tp, body } => {
            env.push(param.clone());
            let b = lower(body, env);
            env.pop();
            DBExpr::Lam(tp.clone(), Box::new(b))
        }

        Expr::App(f, x) => DBExpr::App(Box::new(lower(f, env)), Box::new(lower(x, env))),

        Expr::Let { name, value, body } => {
            let v = lower(value, env);
            env.push(name.clone());
            let b = lower(body, env);
            env.pop();
            DBExpr::Let(Box::new(v), Box::new(b))
        }

        Expr::IsZero(expr) => DBExpr::IsZero(Box::new(lower(expr, env))),
        Expr::Pred(expr) => DBExpr::Pred(Box::new(lower(expr, env))),
    }
}

fn index_type(n: usize) -> proc_macro2::TokenStream {
    let mut ts = quote::quote! { I0 };
    for _ in 0..n {
        ts = quote::quote! { ISucc<#ts> };
    }
    ts
}

fn expand_type(tp: &Tp) -> proc_macro2::TokenStream {
    match tp {
        Tp::Bool => quote::quote! { Bool },
        Tp::Nat => quote::quote! { Nat },
        Tp::Arrow(from, to) => {
            let from = expand_type(from);
            let to = expand_type(to);
            quote::quote! { Arrow<#from, #to> }
        }
    }
}

impl DBExpr {
    fn expand(&self) -> proc_macro2::TokenStream {
        match self {
            DBExpr::True => quote::quote! { True },
            DBExpr::False => quote::quote! { False },
            DBExpr::Nat(n) => {
                let mut ts = quote::quote! { Zero };
                for _ in 0..*n {
                    ts = quote::quote! { Succ<#ts> };
                }
                ts
            }

            DBExpr::Var(i) => {
                let idx = index_type(*i);
                quote::quote! { Var<#idx> }
            }

            DBExpr::If(c, t, e) => {
                let c = c.expand();
                let t = t.expand();
                let e = e.expand();
                quote::quote! {
                    If<#c, #t, #e>
                }
            }

            DBExpr::Lam(tp, body) => {
                let b = body.expand();
                let tp_tokens = expand_type(tp);
                quote::quote! {
                    Lam<#tp_tokens, #b>
                }
            }

            DBExpr::App(f, x) => {
                let f = f.expand();
                let x = x.expand();
                quote::quote! {
                    App<#f, #x>
                }
            }

            DBExpr::Let(v, b) => {
                let v = v.expand();
                let b = b.expand();
                quote::quote! {
                    Let<#v, #b>
                }
            }

            DBExpr::IsZero(expr) => {
                let e = expr.expand();
                quote::quote! {
                    IsZero<#e>
                }
            }

            DBExpr::Pred(expr) => {
                let e = expr.expand();
                quote::quote! {
                    Pred<#e>
                }
            }
        }
    }
}

#[proc_macro]
pub fn stlc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expr = syn::parse_macro_input!(input as Expr);

    let mut env = Vec::new();
    let db = lower(&expr, &mut env);
    let ty = db.expand();

    proc_macro::TokenStream::from(quote::quote! {
        #ty
    })
}
