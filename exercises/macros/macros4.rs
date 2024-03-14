// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

/// macro_rules 是 Rust 中用于定义宏的宏，它允许你定义一个或多个宏，
/// 每个宏都可以匹配一个特定的语法模式，并生成一些代码。
/// #[rustfmt::skip]是一个属性，用于跳过代码的自动格式化。
/// 在Rust中，rustfmt是一个工具，用于自动格式化Rust代码，使其更易读。
/// 当在代码中使用#[rustfmt::skip]属性时，rustfmt将不会对这段代码进行格式化。
/// 这对于保持代码的一致性非常有用，特别是当你需要对一些无法自动格式化的代码进行手动修改时。
/// 需要注意的是，#[rustfmt::skip]属性仅适用于rustfmt工具。
/// 如果你使用的是其他代码格式化工具，可能需要使用不同的属性或选项来实现类似的功能。
// #[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    // ($val:expr,) 表示一个带有一个参数的表达式。
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
