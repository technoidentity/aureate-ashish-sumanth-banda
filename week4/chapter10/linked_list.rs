struct ListNode<T> {
    value: T,
    next: Option<Box<ListNode<T>>>,
}
struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
}
impl<T> LinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }
    fn push_front(&mut self, value: T) {
        let old_head = self.head.take();
        let new_node = Box::new(ListNode {
            value,
            next: old_head,
        });
        self.head = Some(new_node);
    }
    fn front(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.value),
            None => None,
        }
    }
}
fn main() {
    let mut list = LinkedList::new();
    println!("Initially: {:?}", list.front());
    list.push_front(30);
    println!("After adding 30: {:?}", list.front());
    list.push_front(20);
    println!("After adding 20: {:?}", list.front());
    list.push_front(10);
    println!("After adding 10: {:?}", list.front());
    assert_eq!(list.front(), Some(&10));
}