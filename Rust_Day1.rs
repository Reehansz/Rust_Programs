fn
main(){
   println!("Rust says Hello to TutorialsPoint !!");
}

---------Comments in Rust

---------Data Types

---Scalar Types

-Integer

fn main() {
   let result = 10;    
   let age:u32 = 20;
   let sum:i32 = 5-15;
   let mark:isize = 10;
   let count:usize = 30;
   
   println!("result value is {}",result);
   println!("sum is {} and age is {}",sum,age);
   println!("mark is {} and count is {}",mark,count);
}


fn main() {
   let age:u8 = 255;
   let weight:u8 = 256;   
   let height:u8 = 257;   
   let score:u8 = 258;    

   println!("age is {} ",age);
   println!("weight is {}",weight);
   println!("height is {}",height);
   println!("score is {}",score);
}


-Floating-point

fn main() {
   let result = 10.00;      
   let interest:f32 = 8.35;
   //let cost:f64 = 15_000.600;  
   
   
   let coordinates: (i32,i32) = (2,3);
   
   println!("result value is {}",result);
   println!("interest is {}",interest);
   println!("cost is {}",cost);
}


-Number Separator

fn main() {
   let float_with_separator = 11_000.555_001;
   println!("float value {}",float_with_separator);
   
   let int_with_separator = 50_000;
   println!("int value {}",int_with_separator);
}


-Booleans & Characters

fn main() {
   let isfun:bool = true;
   let special_character = '@';
   let alphabet:char = 'A';
   let emoji:char = '😁';
   
   println!("SC {}",special_character);
   println!("alphabet is {}",alphabet);
   println!("emoji is {}",emoji);
   println!("PES ? {}",isfun);
}

fn main() {
	let mut a=10
	a=20
	println!("a={}",a)
	
	const mut a=10
	const a=20
	println!("a={}",a)
	
}

-Characters

fn main() {
   let special_character = '@';
   let alphabet:char = 'A';
   let emoji:char = '😁';
   
   println!("special character is {}",special_character);
   println!("alphabet is {}",alphabet);
   println!("emoji is {}",emoji);
}



---------Variables

fn main() {
   let fees = 25_000;
   let salary:f64 = 35_000.00;
   println!("fees is {} and salary is {}",fees,salary);
}

fn main() {
   let fees = 25_000;
   println!("fees is {} ",fees);
   fees = 35_000;
   println!("fees changed is {}",fees);
}


fn main() {
   let mut fees:i32 = 25_000;
   println!("fees is {} ",fees);
   fees = 35_000;
   println!("fees changed is {}",fees);
}


---------Constant

fn main() {
   const USER_LIMIT:i32 = 100;    
   const PI:f32 = 3.14;           

   println!("user limit is {}",USER_LIMIT);  
   println!("pi value is {}",PI);           
}


---------Shadowing of Variables and Constants

fn main() {
   let salary = 100.00;
   let salary = 1.50 ; 
   println!("The value of salary is :{}",salary);
}


fn main() {
   let uname = "PESU";
   let uname = uname.len();
   println!("name changed to integer : {}",uname);
}


----------String

---String Literal(&str)

fn main() {
   let company:&str="PES";
   let location:&str = "Bengaluru";
   println!("company is : {} location :{}",company,location);
}


----String Object(String)

fn main(){
   let empty_string = String::new();
   println!("length is {}",empty_string.len());

   let content_string = String::from("PES");
   println!("length is {}",content_string.len());
}


-to_string()

fn main(){
   let name1 = "PES ,   Bengaluru!".to_string();
   println!("{}",name1);
}


-replace()

fn main(){
   let name1 = "Hello PES".to_string();         
   let name2 = name1.replace("PES","PESU");   
   println!("{}",name2);
}

-push()

fn main(){
   let mut company = "PES".to_string();
   company.push('U');
   println!("{}",company);
}

-push_str()

fn main(){
   let mut company = "PES".to_string();
   company.push_str(" University");
   println!("{}",company);
}


-trim()

fn main() {
   let fullname = "PES University";
   println!("Before trim ");
   println!("length is {}",fullname.len());
   println!();
   println!("After trim ");
   println!("length is {}",fullname.trim().len());
}