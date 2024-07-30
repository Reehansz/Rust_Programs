fn main(){
    let nums=[1,3,5,-3,4,-4,-9];
    let mut sum=0;let mut prod=1;
    for n in nums{
        if n>0{
            sum=sum+n;
        }
        else{
            prod=prod*n
        }

        
    }
    println!("Sum of positive numbers: {}\nProduct of negative numbers: {}",sum ,prod);
}