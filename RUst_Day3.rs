Functions

fn greet() {
    println!("Hello, World!");
}

fn main() {
    greet();
}

--------------

fn add() {
    let a = 5;
    let b = 10;

    let sum = a + b;

    println!("Sum of a and b = {}", sum);
}

fn main() {
    add();
}


---------------

fn add(a: i32, b: i32)->i32 {
    let sum = a + b;
     sum
}

fn main() {
    add(2, 11);
    println!("Sum of a and b = {}", sum);
	
}


--------------

fn add(a: i32, b: i32) -> i32 {
    let sum = a + b;

    return sum;
}

fn main() {
    let sum = add(3, 5);

    println!("Sum of a and b = {}", sum);
}

--------------
Closure 


fn main() {
    let print_text = |a:i32,b:i32| {
		println!("Hello, World!");
    }
	
    print_text(2,3); 
}

---------------


fn main() {
    let add_one = |x: i32| x + 1;
    
    let result = add_one(2);
    
    println!("Result = {}", result);
}

-----------------

fn main() {
    let squared_sum = |x: i32, y: i32| {

        let mut sum: i32 = x + y;

        let mut result: i32 = sum * sum;
        
        return result;
    };

    let result = squared_sum(5, 3);
    
    println!("Result = {}", result);
}

----------------------------

Tuple

fn main() {
   let t1:(i32,f64,u8) = (-325,4.9,22);
   println!("Values are :{:?}",t1);
   println!("float is :{}",t1.1);
   println!("unsigned integer is :{}",t1.2);
}

--------------------

fn main(){
   let b:(i32,bool,f64) = (110,true,10.9);
   print(b);
}

fn print(x:(i32,bool,f64)){
   println!("Inside print method");
   println!("{:?}",x);
}

---------------------------------------

Destructing

fn main(){
   let b:(i32,bool,f64) = (30,true,7.9);
   display(b);
}

fn display(x:(i32,bool,f64)){
   println!("Inside print method");
   
   let (age,is_male,cgpa) = x; 
   
   println!("Age is {} , isMale? {},cgpa is {}",age,is_male,cgpa);
}

------------------------------------------------

Array

fn main(){
   let arr:[i32;4] = [10,20,30,40];
   println!("array is {:?}",arr);
}

-------------- Default

fn main() {
   let arr:[i32;4] = [-1;4];
   println!("array is {:?}",arr);
}

------------------Loops

fn main(){
   let arr:[i32;4] = [10,20,30,40];
   println!("array is {:?}",arr);

   for index in 0..4 {
      println!("index is: {} & value is : {}",index,arr[index]);
   }
}

--------------------iter

fn main(){

let arr:[i32;4] = [10,20,30,40];
   println!("array is {:?}",arr);

   for val in arr.iter(){
      println!("value is :{}",val);
   }
}




fn main() {
    let numbers = [2, 1, 17, 99, 34, 56];
    
    let numbers_iterator = numbers.iter();
    
    for number in numbers_iterator {
        println!("{}", number);
    }
}


--------------------PV

fn main() {
   let arr = [10,20,30];
   update(arr);

   print!("Inside main {:?}",arr);
}
fn update(mut arr:[i32;3]){
   for i in 0..3 {
      arr[i] = 0;
   }
   println!("Inside update {:?}",arr);
}

---------------------PR

fn main() {
   let mut arr = [10,20,30];
   update(&mut arr);
   print!("Inside main {:?}",arr);
}
fn update(arr:&mut [i32;3]){
   for i in 0..3 {
      arr[i] = 0;
   }
   println!("Inside update {:?}",arr);
}

------------

fn main() {
    let numbers = create_array();
    println!("Array: {:?}", numbers);
}

fn create_array() -> [i32; 5] {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr
}
