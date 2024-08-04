Collections--------------------------

-----------Vector

fn main() {
   let v = vec![1,2,3];
   println!("{:?}",v);
}


fn main() {
   let v = Vec::new();

   println!("{:?}",v);
}


fn main() {
   let v = Vec::new();
   v.push(20);
   v.push(30);
   v.push(40);

   println!("{:?}",v);
}


fn main() {
   let mut v = vec![10,20,30];
   v.remove(1);
   println!("{:?}",v);
}


fn main() {
   let v = vec![10,20,30];
   if v.contains(&10) {
      println!("found 10");
   }
   println!("{:?}",v);
}



----------------------------------

HashMap



-------------------------------


use std::collections::HashMap;

fn main() {
    let mut info = HashMap::new();
    
    println!("HashMap = {:?}", info);
}


-------------------------

use std::collections::HashMap;

fn main(){
   let mut stateCodes = HashMap::new();
   stateCodes.insert("KL","Kerala");
   stateCodes.insert("MH","Maharashtra");
   println!("{:?}",stateCodes);
   println!("size of map is {}",stateCodes.len());
}


-------------------------------


use std::collections::HashMap;
fn main() {
   let mut stateCodes = HashMap::new();
   stateCodes.insert("KL","Kerala");
   stateCodes.insert("MH","Maharashtra");
   println!("size of map is {}",stateCodes.len());
   println!("{:?}",stateCodes);

   match stateCodes.get(&"KL") {
      Some(value)=> {
         println!("Value for key KL is {}",value);
      }
      None => {
         println!("nothing found");
      }
   }
}

-------------------------

use std::collections::HashMap;
fn main() {
   let mut stateCodes = HashMap::new();
   stateCodes.insert("KL","Kerala");
   stateCodes.insert("MH","Maharashtra");

   for (key, val) in stateCodes.iter() {
      println!("key: {} val: {}", key, val);
   }
}

-----------------------------


use std::collections::HashMap;
fn main() {
   let mut stateCodes = HashMap::new();
   stateCodes.insert("KL","Kerala");
   stateCodes.insert("MH","Maharashtra");
   stateCodes.insert("GJ","Gujarat");

   if stateCodes.contains_key(&"GJ") {
      println!("found key");
   }
   
   stateCodes.remove(&"MH");
   println!("length of the hashmap after remove() {}",stateCodes.len());

}


--------------------


HashSet


use std::collections::HashSet;
fn main() {
   let mut names = HashSet::new();

   names.insert("PES");
   names.insert("University");
   names.insert("Bengaluru");
   names.insert("PES");

   println!("{:?}",names);
   
   println!("length of the Hashset: {}",names.len());
   
   names.remove(&"Bengaluru");
   println!("length of the Hashset after remove() : {}",names.len());
}