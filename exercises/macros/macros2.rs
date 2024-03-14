// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.

/// 在Rust中，#[macro_use]是一个属性，用于从其他模块导入宏。
/// 当在某个模块中使用#[macro_use]属性时，它会将该模块中定义的所有宏导入到当前模块中。
/// 这使得在当前模块中使用这些宏变得非常方便，无需手动导入每个宏。
/// 需要注意的是，#[macro_use]属性仅适用于模块内部定义的宏。
/// 如果要在模块外部使用宏，仍然需要使用use关键字导入宏。
#[macro_use]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
