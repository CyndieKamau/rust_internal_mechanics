#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    //Test empty singly linked list properties and behaviour
    fn test_empty_list_properties() {
        let list = SinglyLinkedList::new();
        assert_eq!(list.head, None);
        assert_eq!(list.tail, None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop(), None);
        assert_eq!(list.iter().count(), 0);
        assert_eq!(list.iter().last(), None);

    }
}