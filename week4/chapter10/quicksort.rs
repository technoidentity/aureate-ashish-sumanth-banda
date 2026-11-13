fn partition<T: Ord>(items: &mut [T]) -> usize {
    let pivot_index = items.len() - 1;

    let mut boundary = 0;

    for current in 0..pivot_index {
        if items[current] < items[pivot_index] {
            items.swap(current, boundary);
            boundary += 1;
        }
    }

    items.swap(boundary, pivot_index);

    boundary
}

fn quick_sort<T: Ord>(items: &mut [T]) {
    if items.len() <= 1 {
        return;
    }

    let pivot_index = partition(items);

    quick_sort(&mut items[..pivot_index]);

    quick_sort(&mut items[pivot_index + 1..]);
}

fn main() {
    let mut numbers = vec![40, 10, 30, 20];

    quick_sort(&mut numbers);
    println!("Sorted: {:?}", numbers);

    assert_eq!(numbers, vec![10, 20, 30, 40]);

    let mut empty: Vec<i32> = vec![];
    quick_sort(&mut empty);
    assert_eq!(empty, vec![]);

    let mut single = vec![7];
    quick_sort(&mut single);
    assert_eq!(single, vec![7]);

    let mut duplicates = vec![3, 1, 3, 2, 1];
    quick_sort(&mut duplicates);
    assert_eq!(duplicates, vec![1, 1, 2, 3, 3]);

    let mut reversed = vec![5, 4, 3, 2, 1];
    quick_sort(&mut reversed);
    assert_eq!(reversed, vec![1, 2, 3, 4, 5]);

    let mut letters = vec!['z', 'a', 'm'];
    quick_sort(&mut letters);
    assert_eq!(letters, vec!['a', 'm', 'z']);

    println!("All quicksort checks passed!");
}