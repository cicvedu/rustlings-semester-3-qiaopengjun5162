// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.

use std::borrow::Cow;

/// 这段Rust代码定义了一个名为`abs_all`的函数，
/// 它接受一个可变引用`input`，类型为`Cow<'b, [i32]>`，
/// 并返回一个可变引用`&'a mut Cow<'b, [i32]>`。
/// `Cow`是一个智能指针，表示“Copy on Write”数据结构，
/// 它可以减少不必要的内存分配和复制。

/// 函数的主要目的是将输入数组中的所有负数转换为正数。
/// 具体实现是遍历输入数组，对于每个元素，如果它是一个负数，就将其转换为正数。
/// 在这个过程中，可能会涉及到对数组的修改，因此需要使用可变引用。

/// 需要注意的是，这个函数使用了泛型参数`'a`和`'b`，
/// 它们分别表示输入数组和输出数组的 lifetime。
/// 在 Rust 中，lifetime 是一个重要的概念，它可以帮助编译器检查代码的正确性。
/// 在这个函数中，`'a`和`'b`的 lifetime 应该相同，因为输入数组和输出数组是共享的。

/// 另外，这个函数使用了`to_mut`方法来获取输出数组的可变引用，
/// 这可能会导致不必要的内存分配。在实际应用中，可以考虑使用其他方法来避免这种情况。

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected borrowed value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` returns a reference to the same data as
        // before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }
}
