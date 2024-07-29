//define two string constants concatenate the two 

// fn main() {
//     let first_name = "Pes".to_string();
//     let last_name = "Universtity".to_string();
//     let full_name = format!("{} {}",first_name,last_name);
//     println!("Full name is {} ", full_name);
// }

// fn main() {
//     let secret_message = "Reehan Shaveez".to_string();
//     let encrypt_message = encrypt_mess(secret_message,6);
//     println!("{}",encrypt_message);

// }

// fn encrypt_mess(secret_message:String,key:u8) -> String {
//     let mut temp_sting = String::from("");
//     for i in secret_message.chars(){
//         temp_sting.push(((i as u8) + key) as char);
//     }
//     return temp_sting;


// }

// fn main() {
//     let word1:&str = "Reehan";
//     let word2:&str = "Shaveez";
//     if word1.len() > word2.len(){
//         println!("{} > then {}",word1, word2);
//     }
//     else if word1.len() == word2.len(){
//         println!("{} = {}",word1,word2);
//     }
//     else{
//         println!("{} < {} ", word1,word2);
//     }
// }

// fn main() {
//     let base_string:&str = "Reehan";
//     let repetations = 4;
//     let repeted_string = base_string.repeat(repetations);
//     println!("{}",repeted_string);

// }

fn main() {
    let message1 = "Reehan".to_string();
    let message2 = "Shaveez".to_string();
    let separator = "|".to_string();
    let num_separator = 4;

    let final_mixed_message = message1 + &separator.repeat(num_separator) + &message2;
    println!("{}",final_mixed_message);
}