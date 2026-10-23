fn binary_search< T:Ord >(items: &[T], target: &T)->Option<usize>{
    let mut left = 0;
    let mut right = items.len();
    while left < right{
        let middle = left + (right - left)/2;
        if &items[middle] == target{
            return Some(middle);
        }else if &items[middle]<target{
            left = middle + 1;
        } else{
            right = middle;
        }

    }
    None
}
fn main() {
    let numbers = vec![10, 20, 30, 40, 50, 60, 70];

    assert_eq!(binary_search(&numbers, &10), Some(0));
    assert_eq!(binary_search(&numbers, &40), Some(3));
    assert_eq!(binary_search(&numbers, &70), Some(6));

    assert_eq!(binary_search(&numbers, &5), None);
    assert_eq!(binary_search(&numbers, &35), None);
    assert_eq!(binary_search(&numbers, &100), None);

    let empty: Vec<i32> = vec![];
    assert_eq!(binary_search(&empty, &10), None);

    // A collection with just one element.
    let single = vec![25];
    assert_eq!(binary_search(&single, &25), Some(0));
    assert_eq!(binary_search(&single, &30), None);

    let letters = vec!['a', 'c', 'e'];
    assert_eq!(binary_search(&letters, &'c'), Some(1));

    println!("All binary search checks passed!");
}
