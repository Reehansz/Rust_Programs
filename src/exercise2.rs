fn check(word:&str)->bool{
    let chars: Vec<char> = word.chars().collect();
    let mut left=0;
    let mut right=word.len()-1 as usize;
    while left<right{
        if chars[left as usize]!=chars[right as usize]{
            return false;
        }
        left+=1;
        right-=1;
    }
    return true;
}

fn main(){
let test_cases = ["racecar", "hello", "madam", "world", "level"];
for i in test_cases{
    println!("{} : {}",i,check(&i))
}}