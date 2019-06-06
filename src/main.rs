use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\n\nLets practice maths!");
    println!("* * * * * * * * * * * * * * *");

    let max_tables = 12;

    let target_correct_answers = 10;
    let mut correct_answers = 0;
    let mut to_review: Vec<String> = [].to_vec();

    println!(
        "\nLets practice until you get {} correct answers",
        target_correct_answers
    );

    loop {
        to_review.dedup_by(|a, b| a.eq_ignore_ascii_case(b));

        if correct_answers >= target_correct_answers {
            let wrong_answers = to_review.len();
            println!("you win! great maths skills!");
            if wrong_answers > 0 {
                println!(
                    "you answered {} correctly and only answered {} wrong! {}",
                    correct_answers,
                    wrong_answers,
                    if (wrong_answers as f32 / correct_answers as f32) < 0.5 {
                        generate_random_goodjob()
                    } else {
                        ""
                    }
                );
            } else {
                println!(
                    "You answered all {} correctly! {}",
                    correct_answers,
                    generate_random_goodjob()
                );
            }

            if wrong_answers > 0 {
                println!("\nLets review these:");

                for review in to_review {
                    println!("{}", review);
                }
            }
            break;
        }

        if correct_answers > 0 {
            println!(
                "you've gotten {} correct answer{}! {}",
                correct_answers,
                if correct_answers == 1 { "" } else { "s" },
                generate_random_goodjob()
            );
        }

        let multiplicand = rand::thread_rng().gen_range(0, max_tables);
        let multiplier = rand::thread_rng().gen_range(0, max_tables);
        let answer = multiplicand * multiplier;

        loop {
            println!("\n");
            println!(
                "#{}:   what is {} x️ {} ?",
                correct_answers + 1,
                multiplicand,
                multiplier
            );
            println!("------------------------------------------");

            let guess = loop {
                let mut guess = String::new();

                // not sure how to work with this as a result, also not sure how to make this fail to try things.
                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");

                let guess: i32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("I didn't understand that, please try again");
                        continue;
                    }
                };

                break guess;
            };

            match guess.cmp(&answer) {
                Ordering::Equal => {
                    println!("Correct! {} x️ {} = {}", multiplicand, multiplier, answer);
                    correct_answers += 1;
                    break;
                }
                _ => {
                    println!("sorry, that answer isn't correct. Try again.");
                    if multiplier == 1 || multiplicand == 1 {
                        println!("remember, anything times 1 equals that number...");
                    } else if multiplicand == 0 || multiplier == 0 {
                        println!("remember, anything times 0 is 0...");
                    } else if multiplicand == 2 || multiplier == 2 {
                        println!("remember, anything times 2 must be an even number...");
                    } else if multiplicand == 5 || multiplier == 5 {
                        println!("remember, anything times 5 is going to end in a 5 or a 0...");
                    } else if multiplicand == 10 || multiplier == 10 {
                        println!("remember, anything times 10 will add a 0 to the right of that number...");
                    }

                    to_review.push(format!("{} x {} = {}", multiplicand, multiplier, answer));
                    continue;
                }
            }
        }
    }
}

fn generate_random_goodjob() -> &'static str {
    let good_jobs: Vec<&str> = vec![
        "Great Job!",
        "Excellent!",
        "Keep it up!",
        "Way to go!",
        "Great Work!",
        "Super!",
        "Wow!",
        "Amazing!",
        "Nice!",
    ];
    good_jobs[rand::thread_rng().gen_range(0, good_jobs.len())]
}
