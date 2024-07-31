use std::collections::HashSet;

fn main() {
   
    let set1: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set2: HashSet<i32> = [4, 5, 6, 7, 8].iter().cloned().collect();

    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    println!("Union: {:?}", union);

    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    println!("Intersection: {:?}", intersection);

    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    println!("Difference (set1 - set2): {:?}", difference);

    let difference2: HashSet<_> = set2.difference(&set1).cloned().collect();
    println!("Difference (set2 - set1): {:?}", difference2);
}
