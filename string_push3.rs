fn main() {
    // let mut z = String::new();
    // z.push_str("hello");
    // println!("{}",z);

    //replace
    // let name1 = "Hello PES University, Hello!".to_string();
    // let name2 = name1.replace("Hello","Welcome");
    // println!("{} {}",name1,name2);

    //as_str()
    // let example_string = String::from("example_string");
    // print_literal(example_string.as_str());

    //trim
    // let fullname = "Pes University\r\n";
    // println!("Before trim ");
    // println!("length is {}",fullname.len());
    // println!();
    // println!("After trim");
    // println!("lenght is {} ", fullname.trim().len());
    // println!("After trim string is {} ", fullname);

    //split_whitespace()

    // let msg = "Pes Universtiy".to_string();
    // let mut i = 1;
    // for token in msg.split_whitespace(){
    //     println!("token {} {}",i,token);
    //     i+=1
    // }

    //split()

    // let fullname = "PES,University,Bangalore";
    // for token in fullname.split(","){
    //     println!("token is {}",token);
    // }

    //store in a vector
    // println!("\n");
    // let tokens:Vec<&str> = fullname.split(",").collect();
    // println!("first part of the string is {} ",tokens[0]);
    // println!("Second part of the string {} ",tokens[1]);

    //type casting
    //converting a number to string
    // let number = 2020;
    // let number_as_string = number.to_string();
    // println!("{}",number_as_string);
    // println!("{}",number_as_string=="2020");

    //concatenate using +
    let n1 = "PES".to_string();
    let n2 = "University".to_string();
    let n3 = n1+" "+&n2;
    println!("{}",n3);

    //concatenate using format macro
    // let n1 = "PES".to_string();
    // let n2 = "University".to_string();
    // let n3 = format!("{} {} Bangalore",n1,n2);
    // println!("{}",n3);




}

fn print_literal(data:&str){
    println!("displaying string literal {}",data);
}