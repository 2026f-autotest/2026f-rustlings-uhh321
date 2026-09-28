// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.





// 1. 删除了 use std::f32;

fn main() {
    // 2. 使用标准库的 PI 常量
    let pi = std::f32::consts::PI; 
    let radius = 5.00f32;

    // 3. 将 f32::powi(radius, 2) 改为 radius.powi(2)
    let area = pi * radius.powi(2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}