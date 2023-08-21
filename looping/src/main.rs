use std::io;
/*
Description: Looping

Instructions
Write a program that prints a riddle, receives input from the user and checks that the answer is correct.

The program must allow an indefinite number of trials and only quit after the correct answer is given.

Every time the user introduces an incorrect answer the program must print the riddle again and after the user gives the correct answer the program must print the number of tries that took to get the correct answer.

Riddle: I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?

Answer: The letter e 
 */
fn main(){
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "The letter e";
    let mut tries = 0;

    loop {
        println!("{}", riddle);

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        if user_input.trim() == answer {
            tries += 1;
            println!("Number of trials: {}", tries);
            break;
        } else {
            tries += 1;
        }
    }
}

#[cfg(test)]
mod tests;