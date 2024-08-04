// fn main() {
//     let v = vec![1,2,3];
//     println!("{:?}",v);
//  }


// fn main() {
//     let v = Vec::new();
 
//     println!("{:?}",v);
//  }

// fn main() {
//     let mut v = Vec::new();
//     v.push(20);
//     v.push(30);
//     v.push(40);
//     v.remove(1);
 
//     println!("{:?}",v);
//  }

// fn main() {
//     let v = vec![10,20,30];
//     if v.contains(&10) {
//        println!("found 10");
//     }
//     println!("{:?}",v);
//  }
use std::collections::HashMap;
// fn main() {
//     let mut info= HashMap::new();
    
//     println!("HashMap = {:?}", info);
// }


// fn main(){
//     let mut stateCodes = HashMap::new();
//     stateCodes.insert("KL","Kerala");
//     stateCodes.insert("MH","Maharashtra");
//     println!("{:?}",stateCodes);
//     println!("size of map is {}",stateCodes.len());
//  }


// use std::collections::HashSet;
// fn main() {
//    let mut names = HashSet::new();

//    names.insert("PES");
//    names.insert("University");
//    names.insert("Bengaluru");
//    names.insert("PES");

//    println!("{:?}",names);
   
//    println!("length of the Hashset: {}",names.len());
   
//    names.remove(&"Bengaluru");
//    println!("length of the Hashset after remove() : {}",names.len());
// }

fn main() {
    let add_one = |x: i32| x + 1;
    
    let result = add_one(2);
    
    println!("Result = {}", result);
}
