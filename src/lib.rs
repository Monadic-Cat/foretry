pub use foretry_impl::{async_ltry, async_try, try_block, cont, brk};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
