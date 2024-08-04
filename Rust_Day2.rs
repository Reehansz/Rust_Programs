If Statement

fn main(){
   let num:i32 = 5;
   if num > 0 {
      println!("number is positive") ;
   }
}


if else statement

fn main() {
   let num = 12;
   if num % 2==0 {
      println!("Even");
   } else {
      println!("Odd");
   }
}


Nested If

fn main() {
   let num = 2 ;
   if num > 0 {
      println!("{} is positive",num);
   } else if num < 0 {
      println!("{} is negative",num);
   } else {
      println!("{} is neither positive nor negative",num) ;
   }
}


Match Statement

fn main(){
   let state_code = "MH";
   match state_code {
      90..100 
	  90..=100
	  
	  => {println!("Found match for MH"); "Maharashtra"},
      "KL" => "Kerala",
      "KA" => "Karnataka",
      "GA" => "Goa",
      _ => "Unknown"
   };
   println!("State name is {}",state);
}



Using "if in a let" statement

fn main()  
  
 let a=if true  
       {  
          1  
       }  
       else  
       {  
           2  
       };  
  
 println!("value of a is: {}", a);  
 
 
 
Loop

Definite Loop

For Loop

fn main(){
   for x in 1..11.rev(){ 
      if x==5 {
         continue;
      }
      println!("x is {}",x);
   }
}


fn main()  
{  
	let result;  
	for i in 1..11  
	{  
		result=2*i;  
		println!("2*{}={}",i,result);  
	}  
}


fn main()  
{   
	 let fruits=["mango","apple","banana","litchi","watermelon"];  
	 for a in fruits.iter()  
	 {  
	   print!("{} ",a);  
	 }   
}   


While

fn main(){
   let x = 0;
   while x < 10{
      x+=1;
      println!("inside loop x value is {}",x);
   }
   println!("outside loop x value is {}",x);
}


Loop

fn main(){
   //while true

   let x = 0;
   loop {
      x+=1;
      println!("x={}",x);

      if x==15 {
         break;
      }
   }
}



//Q2

fn main() {
    let input = "hello";
    let mut reversed = String::new();
	
    let input_bytes = input.as_bytes();

    for i in (0..input.len()).rev() {
        reversed.push(input_bytes[i] as char);
    }

    println!("Original string: {}", input);
    println!("Reversed string: {}", reversed);
}