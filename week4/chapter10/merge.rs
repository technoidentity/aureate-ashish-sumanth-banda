fn merge<T:Ord + Clone> (left: &[T],right: &[T])-> Vec<T>{
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < left.len() && j<right.len(){
        if left[i] <= right[j]{
            result.push(left[i].clone());
            i += 1;
        }else{
            result.push(right[j].clone());
            j += 1;
        }
    }
    while i < left.len(){
        result.push(left[i].clone());
        i += 1;
    }
    while j < right.len(){
        result.push(right[j].clone());
        j += 1;
    }
    result
}

fn merge_sort<T: Ord +Clone>(items:&[T])-> Vec<T>{
    if items.len()<=1{
        return items.to_vec();
    }
    let middle = items.len()/2;
    let left = merge_sort(&items[..middle]);
    let right = merge_sort(&items[middle..]);
    merge(&left, &right)
}

fn main(){
    let numbers = vec![40,10,30,20];
    let sorted = merge_sort(&numbers);
    println!("Original:{:?}",numbers);
    println!("Sorted: {:?}",sorted);

    assert_eq!(sorted,vec![10,20,30,40]);
    let empty:Vec<i32> = vec![];
    assert_eq!(merge_sort(&empty),vec![]);
    assert_eq!(merge_sort(&[7]),vec![7]);
    assert_eq!(merge_sort(&[3,1,3,2,1]),vec![1,1,2,3,3]);
    assert_eq!(merge_sort(&[1,2,3]),vec![1,2,3]);
    assert_eq!(merge_sort(&[3,2,1]),vec![1,2,3]);
    let letters = vec!['z','a','m'];
    assert_eq!(merge_sort(&letters),vec!['a','m','z']);
    println!("All merge sort checks passed!");
}