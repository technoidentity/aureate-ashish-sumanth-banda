#[derive(Debug, Clone, PartialEq, Eq)]
pub enum List<T> {
    Empty,
    Node(T, Box<List<T>>),
}

impl<T> List<T> {
    pub fn new() -> Self {
        List::Empty
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, List::Empty)
    }

    pub fn head(&self) -> Option<&T> {
        match self {
            List::Empty => None,
            List::Node(value, _) => Some(value),
        }
    }

    pub fn tail(&self) -> Option<&List<T>> {
        match self {
            List::Empty => None,
            List::Node(_, rest) => Some(rest.as_ref()),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            List::Empty => 0,
            List::Node(_, rest) => 1 + rest.len(),
        }
    }
}

impl<T: Clone> List<T> {
    pub fn insert(&self, value: T) -> Self {
        List::Node(value, Box::new(self.clone()))
    }

    pub fn reverse(&self) -> Self {
        self.reverse_into(List::Empty)
    }

    fn reverse_into(&self, reversed: Self) -> Self {
        match self {
            List::Empty => reversed,
            List::Node(value, rest) => {
                let next = List::Node(value.clone(), Box::new(reversed));
                rest.reverse_into(next)
            }
        }
    }
}

impl<T: Clone + PartialEq> List<T> {
    pub fn remove(&self, target: &T) -> Self {
        match self {
            List::Empty => List::Empty,

            // Stop after removing the first matching node.
            List::Node(value, rest) if value == target => {
                rest.as_ref().clone()
            }

            List::Node(value, rest) => {
                List::Node(value.clone(), Box::new(rest.remove(target)))
            }
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    fn fixture<T: Clone>(values: &[T]) -> List<T> {
        match values.split_first() {
            None => List::Empty,
            Some((first, rest)) => {
                List::Node(first.clone(), Box::new(fixture(rest)))
            }
        }
    }

    #[test]
    fn empty_list_has_no_elements() {
        let list = List::<i32>::new();

        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert_eq!(list.head(), None);
        assert_eq!(list.tail(), None);
        assert_eq!(List::<i32>::default(), list);
    }

    #[test]
    fn reads_head_tail_and_length() {
        let list = fixture(&[10, 20, 30]);
        let expected_tail = fixture(&[20, 30]);

        assert!(!list.is_empty());
        assert_eq!(list.len(), 3);
        assert_eq!(list.head(), Some(&10));
        assert_eq!(list.tail(), Some(&expected_tail));
    }

    #[test]
    fn single_element_has_an_empty_tail() {
        let list = fixture(&[10]);
        let empty = List::Empty;

        assert_eq!(list.len(), 1);
        assert_eq!(list.head(), Some(&10));
        assert_eq!(list.tail(), Some(&empty));
    }

    #[test]
    fn inserts_into_an_empty_list() {
        let original = List::new();

        let updated = original.insert(10);

        assert_eq!(updated, fixture(&[10]));
        assert!(original.is_empty());
    }

    #[test]
    fn inserts_at_front_and_preserves_original() {
        let original = fixture(&[10, 20]);

        let updated = original.insert(5);

        assert_eq!(updated, fixture(&[5, 10, 20]));
        assert_eq!(original, fixture(&[10, 20]));
    }

    #[test]
    fn removes_from_front_middle_and_end() {
        let original = fixture(&[10, 20, 30]);

        assert_eq!(original.remove(&10), fixture(&[20, 30]));
        assert_eq!(original.remove(&20), fixture(&[10, 30]));
        assert_eq!(original.remove(&30), fixture(&[10, 20]));

        assert_eq!(original, fixture(&[10, 20, 30]));
    }

    #[test]
    fn removes_only_the_first_matching_value() {
        let original = fixture(&[10, 20, 20, 30]);

        let updated = original.remove(&20);

        assert_eq!(updated, fixture(&[10, 20, 30]));
        assert_eq!(original, fixture(&[10, 20, 20, 30]));
    }

    #[test]
    fn removing_a_missing_value_preserves_contents() {
        let original = fixture(&[10, 20]);

        assert_eq!(original.remove(&99), fixture(&[10, 20]));
        assert_eq!(original, fixture(&[10, 20]));
    }

    #[test]
    fn removes_from_empty_and_single_element_lists() {
        let empty = List::<i32>::new();
        let single = fixture(&[10]);

        assert_eq!(empty.remove(&10), List::Empty);
        assert_eq!(single.remove(&10), List::Empty);
        assert_eq!(single.remove(&99), fixture(&[10]));
        assert_eq!(single, fixture(&[10]));
    }

    #[test]
    fn reverses_and_preserves_original() {
        let original = fixture(&[10, 20, 30]);

        let reversed = original.reverse();

        assert_eq!(reversed, fixture(&[30, 20, 10]));
        assert_eq!(original, fixture(&[10, 20, 30]));
    }

    #[test]
    fn reverses_empty_and_single_element_lists() {
        let empty = List::<i32>::new();
        let single = fixture(&[10]);

        assert_eq!(empty.reverse(), List::Empty);
        assert_eq!(single.reverse(), fixture(&[10]));
    }

    #[test]
    fn reversing_twice_restores_original_contents() {
        let original = fixture(&[10, 20, 20, 30]);

        assert_eq!(original.reverse().reverse(), original);
    }

    #[test]
    fn supports_owned_strings() {
        let original = fixture(&[
            String::from("Ashish"),
            String::from("Kiran"),
        ]);

        let inserted = original.insert(String::from("Meera"));
        let removed = original.remove(&String::from("Ashish"));
        let reversed = original.reverse();

        assert_eq!(
            inserted,
            fixture(&[
                String::from("Meera"),
                String::from("Ashish"),
                String::from("Kiran"),
            ])
        );

        assert_eq!(removed, fixture(&[String::from("Kiran")]));

        assert_eq!(
            reversed,
            fixture(&[
                String::from("Kiran"),
                String::from("Ashish"),
            ])
        );

        assert_eq!(
            original,
            fixture(&[
                String::from("Ashish"),
                String::from("Kiran"),
            ])
        );
    }
}