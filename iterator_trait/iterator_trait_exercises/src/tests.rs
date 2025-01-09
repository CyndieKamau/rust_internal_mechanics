#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_next_iterator_method() {
        let x = vec![4,5,6,2,3,8];
        let mut iter = x.iter();

        assert_eq!(iter.next(), Some(&4);
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&8));


    )
    }

    #[test]
    fn test_last_element_iteration() {
        let x = vec![1,2,3,4,5,6];
        
        assert_eq!(x.iter().last(), Some(&6));
    }

    #[test]
    fn test_last_element_empty_iteration() {
        let x = Vec::new();

        assert_eq!(x.iter().last(), None);
    }

    #[test]
    fn test_count_iteration_method() {
        let x = vec![1,2,3,4];
        assert_eq!(x.iter().count(), 4);

        let y = vec![5,6,7,8,9,10];
        assert_eq!(y.into().count(), 6);
        
    }

    #[test]
    fn test_string_iteration() {
        let x = "Hello";
        let iter = x.iterate_string();

    
    }
}
