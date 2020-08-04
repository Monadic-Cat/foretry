// use proc_macro2::{Span, TokenStream};
// use syn::{parse2, Ident, Block, Type, Token, Result as SynResult};
// use syn::parse::{Parse, ParseStream};
// use quote::quote_spanned;


// struct HappyPath {
//     block: Block,
// }

// struct TryBlock {
//     happy_path_type: Type,
//     type_separator: Token![,],
//     sad_path_type: Type,
//     type_param_end: Token![|],
//     happy_path: HappyPath,
//     sad_path: Block,
// }
// impl Parse for TryBlock {
//     fn parse(input: ParseStream) -> SynResult<Self> {
//         unimplemented!()
//     }
// }

// fn foretry(input: TokenStream) -> TokenStream {

//     unimplemented!()
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


/// An internal control flow effect representation
#[doc(hidden)]
pub enum Return<T> {
    Actual(T),
    Continue,
    Break,
}

/// Try block that forwards `.await`, `break`, and `continue`
/// Must be used in async and loop context.
#[macro_export]
macro_rules! async_ltry {
    ($rty:ty, $ety:ty | $mblk:block catch($ename:ident) $cblk:block) => {{
        let result: Result<$crate::Return<$rty>, $ety> = (|| async { Ok($crate::Return::Actual($mblk)) })().await;
        match result {
            Ok($crate::Return::Actual(x)) => x,
            Ok($crate::Return::Continue) => continue,
            Ok($crate::Return::Break) => break,
            Err($ename) => $cblk,
        }
    }};
    (move $rty:ty, $ety:ty | $mblk:block catch($ename:ident) $cblk:block) => {{
        let result: Result<$crate::Return<$rty>, $ety> =
            (move || async { Ok($crate::Return::Actual($mblk)) })().await;
        match result {
            Ok($crate::Return::Actual(x)) => x,
            Ok($crate::Return::Continue) => continue,
            Ok($crate::Return::Break) => break,
            Err($ename) => $cblk,
        }
    }};
}

/// Try block that forwards `.await`
/// Must be used in async context.
#[macro_export]
macro_rules! async_try {
    ($rty:ty, $ety:ty | $mblk:block catch($ename:ident) $cblk:block) => {{
        let result: Result<$rty, $ety> = (|| async { Ok($mblk) })().await;
        match result {
            Ok(x) => x,
            Err($ename) => $cblk,
        }
    }};
    (move $rty:ty, $ety:ty | $mblk:block catch($ename:ident) $cblk:block) => {{
        let result: Result<$rty, $ety> = (move || async { Ok($mblk) })().await;
        match result {
            Ok(x) => x,
            Err($ename) => $cblk,
        }
    }};
}

/// Try block that doesn't forward any control flow effects
/// Can be used in any context.
#[macro_export]
macro_rules! try_block {
    ($rty:ty, $ety:ty | $mblk:block catch($ename:ident) $cblk:block) => {{
        let result: Result<$rty, $ety> = (|| Ok($mblk))();
        match result {
            Ok(x) => x,
            Err($ename) => $cblk,
        }
    }};
    (move $rty:ty, $ety:ty | $mblk:block catch($ename:ident) $cblk:block) => {{
        let result: Result<$rty, $ety> = (move || Ok($mblk))();
        match result {
            Ok(x) => x,
            Err($ename) => $cblk,
        }
    }};
}

/// Replacement for `continue` inside the provided try block macros.
/// Note that this is not required, and not usable, in the catch block.
#[macro_export]
macro_rules! cont {
    () => {
        return Ok($crate::Return::Continue);
    };
}

/// Replacement for `break` inside the provided try block macros.
/// Note that this is not required, and not usable, in the catch block.
#[macro_export]
macro_rules! brk {
    () => {
        return Ok($crate::Return::Break);
    };
}
