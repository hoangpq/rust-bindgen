/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RefPtr<T> {
    pub use_of_t: T,
}
impl <T> Default for RefPtr<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UsesRefPtrWithAliasedTypeParam<U> {
    pub member: RefPtr<UsesRefPtrWithAliasedTypeParam_V<U>>,
}
pub type UsesRefPtrWithAliasedTypeParam_V<U> = U;
impl <U> Default for UsesRefPtrWithAliasedTypeParam<U> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
