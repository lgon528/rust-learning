use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader};

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    question: String,
    options: Vec<String>,
    answer: String,
}

fn main() -> io::Result<()> {
    let file = File::open("questions.json")?;
    let reader = BufReader::new(file);
    let questions: Vec<Question> = serde_json::from_reader(reader)?;

    let mut score = 0;

    for (i, q) in questions.iter().enumerate() {
        println!("\nQuestion {}: {}", i + 1, q.question);
        for (j, option) in q.options.iter().enumerate() {
            println!("  {}. {}", j + 1, option);
        }

        let mut user_answer = String::new();
        io::stdin().read_line(&mut user_answer)?;
        let user_answer = user_answer.trim();

        if user_answer == q.answer {
            println!("Correct!");
            score += 1;
        } else {
            println!("Incorrect. The correct answer is: {}", q.answer);
        }
    }

    println!("\nYou scored {} out of {}.", score, questions.len());

    Ok(())
}
