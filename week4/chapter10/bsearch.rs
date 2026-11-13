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
fn main(){}

#[cfg(test)]
mod tests{
    use super::binary_search;
    #[test]
    fn finds_first_middle_and_last_elements(){
        let numbers = vec![10,20,30,40,50,60,70];
        assert_eq!(binary_search(&numbers, &10),Some(0));
        assert_eq!(binary_search(&numbers, &40),Some(3));
        assert_eq!(binary_search(&numbers, &70),Some(6));
    }
    #[test]
    fn returns_none_when_target_is_absent(){
        let numbers = vec![10,20,30,40,50,60,70];
        assert_eq!(binary_search(&numbers,&5),None);
        assert_eq!(binary_search(&numbers,&35),None);
        assert_eq!(binary_search(&numbers,&100),None);
    }
    #[test]
    fn finds_target_in_single_element_input(){
        let numbers = vec![25];
        let result = binary_search(&numbers,&25);
        assert_eq!(result,Some(0));
    }
    #[test]
    fn returns_none_when_single_element_does_not_match(){
        let numbers = vec![25];
        assert_eq!(binary_search(&numbers,&20),None);
        assert_eq!(binary_search(&numbers,&30),None);
    }
    #[test]
    fn works_with_characters(){
        let letters = vec!['a','c','e'];
        let result = binary_search(&letters,&'c');
        assert_eq!(result,Some(1));
    }
    #[test]
    fn searches_two_element_input(){
        let numbers = vec![10,20];
        assert_eq!(binary_search(&numbers,&10),Some(0));
        assert_eq!(binary_search(&numbers,&20),Some(1));
        assert_eq!(binary_search(&numbers,&15),None);
    }
    #[test]
    fn finds_a_matching_element_when_duplicates_exit(){
        let numbers = vec![10,20,20,20,30];
        let index = binary_search(&numbers, &20)
            .expect("20 is present, so a matching index should be returned");
        assert_eq!(numbers[index],20);

    }
}