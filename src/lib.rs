#![cfg_attr(docsrs, feature(doc_cfg))]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(feature = "foo")]
#[cfg_attr(docsrs, doc(cfg(feature = "foo")))]
pub fn foo() -> u64 {
    1 + 1
}

pub struct MyStruct {
    pub field: u64
}

#[cfg(feature = "traits")]
#[cfg_attr(docsrs, doc(cfg(feature = "traits")))]
impl Default for MyStruct {
    fn default() -> Self {
        Self { field: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
