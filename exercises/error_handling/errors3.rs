// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!


use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }

    Ok(()) // This says "program terminated without error,"
    // and only happens if total_cost doesn't through its own
    // error first, earlier in main. Previously, main was just
    // returning () -- that's what happens when main exits. But
    // since I updated main's return type to be Result<(), ParseIntError>, the compiler will now look for either an Ok() or an Err(). Well, if total_cost returns an error then we'll be fine, but
    // if not, main is still returning () when it needs to return...Ok()! And what goes inside that Ok()? Well, all that we need is ().
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
