
use std::io;

fn main() {
let mut x = String::new();
let mut y = String::new();
let mut opp = String::new();
println!("Type first number");
io::stdin().read_line(&mut x);
println!("Type second number");
io::stdin().read_line(&mut y);

println!("Type 1 if you wanna Add");
println!("Type 2 if you wanna Subtract");
println!("Type 3 if you wanna Multiply");
println!("Type 4 if you wanna Divide");

    
let mut x = 6;
io:: stdin().read_line(&mut opp);
let x: i32 = x
.trim()
.parse()
.expect("you have typed non-integer first no.");
let y: i32 = y
.trim( )
.parse()
.expect("you have typed non-integer first no.");
let opp: i32 = opp
.trim()
. parse()
.expect("you have to choose between 1,2,3,4");


match opp {
1 =>println!("Sum of numbers = {}", x+y),
2 => println!("Subtraction of numbers ={}", x-y),
3 =>println!( "multiplication of numbers= {}", x*y), 
4 =>println!("Division of numbers = {}", x/y),
_ =>println!("Select the valid operation"),
}

}


