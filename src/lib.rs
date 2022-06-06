#![no_std]
//! A crate providing function which helps specifying type within the expression.

/// Specify the type [`T`] of expression [`expr`]
/// # Example
/// ```
/// # use the_type::the;
/// let foo: u8 = 0;
/// let items1 : [usize; 4] = [foo.into(); 4];
///
/// // won't compile becuase Into is a generic trait but self.into() is not a generic method
/// // let items2 = [foo.into::<usize>(): usize ; 4];
/// // an alternative way: 
/// let items3 = [usize::from(foo); 4];
/// // a more intuitive way:
/// let items4 = [the::<usize>(foo.into()); 4];
/// ```
pub fn the<T>(expr: T) -> T {
    expr
}
