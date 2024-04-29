#[cfg(test)]
mod tests {
    use crate::number_set::NumberSet;

    #[test]
    fn test_new() {
        let number_set = NumberSet::new();
        assert_eq!(number_set.len(), 16);
        for i in 0..16 {
            assert!(number_set.contains(i));
        }
    }

    #[test]
    fn test_remove() {
        let mut number_set = NumberSet::new();
        number_set.remove(3);
        assert_eq!(number_set.len(), 15);
        assert!(!number_set.contains(3));
    }

    #[test]
    fn test_contains() {
        let number_set = NumberSet::new();
        assert!(number_set.contains(0));
        assert!(!number_set.contains(16));
    }

    #[test]
    fn test_single_is_none_too_many() {
        let number_set = NumberSet::new();
        assert_eq!(number_set.single(), None);
    }

    #[test]
    fn test_single_is_none() {
        let mut number_set = NumberSet::new();
        for i in 0..16 {
            number_set.remove(i);
        }
        assert_eq!(number_set.single(), None);
        assert_eq!(number_set.len(), 0);
    }

    #[test]
    fn test_single_is_some() {
        let mut number_set = NumberSet::new();
        for i in 1..16 {
            number_set.remove(i);
        }
        assert_eq!(number_set.single(), Some(0));
        assert_eq!(number_set.len(), 1);
    }

    #[test]
    fn test_first() {
        let number_set = NumberSet::new();
        assert_eq!(number_set.first(), Some(0));
    }

    #[test]
    fn test_without() {
        let number_set = NumberSet::new();
        let number_set = number_set.without(3);
        assert_eq!(number_set.len(), 15);
        assert!(!number_set.contains(3));
    }

    #[test]
    fn test_into_iter() {
        let mut number_set = NumberSet::new();
        number_set.remove(3);
        number_set.remove(5);
        number_set.remove(7);
        let mut iter = number_set.into_iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(11));
        assert_eq!(iter.next(), Some(12));
        assert_eq!(iter.next(), Some(13));
        assert_eq!(iter.next(), Some(14));
        assert_eq!(iter.next(), Some(15));
        assert_eq!(iter.next(), None);
    }
}
