fn main() {
    let array = [2, 3, 4, 5, 8, 9, 10, 11];
    let mid = array.len() / 2;

    let slice1 = &array[..mid]; // First half
    let slice2 = &array[mid..]; // Second half

    let mut sum_odd = 0;
    let mut product_even = 1;

    // Process first slice
    for (i, &value) in slice1.iter().enumerate() {
        if i % 2 == 0 {
            product_even *= value;
        } else {
            sum_odd += value;
        }
    }

    // Process second slice
    for (i, &value) in slice2.iter().enumerate() {
        if i % 2 == 0 {
            product_even *= value;
        } else {
            sum_odd += value;
        }
    }

    println!("Sum of elements at odd indices: {}", sum_odd);
    println!("Product of elements at even indices: {}", product_even);
}