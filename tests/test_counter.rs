#[cfg(test)]
mod tests {

    use counter_fpy::Counter;

    #[test]
    fn test_string_counter() {
        let sample = ["bxffour", "bxffour", "blackprince"];

        let counter = Counter::new();
        let collection = Counter::from(counter, &sample);

        for (key, count) in collection.iter() {
            println!("Key: {key} has count: {count}");
        }
    }

    #[test]
    fn test_integer_counter() {
        let sample = [1, 2, 3, 4, 3, 3, 1, 2, 4, 5, 3, 2];

        let counter = Counter::new();
        let collection = Counter::from(counter, &sample);

        for (key, count) in collection.iter() {
            println!("Key: {key} has count: {count}");
        }
    }

    #[test]
    fn test_get_count() {
        let sample = [1, 2, 3, 4, 3, 3, 1, 2, 4, 5, 3, 2];

        let counter = Counter::new();
        let collection = Counter::from(counter, &sample);

        assert_eq!(collection.get(&1), Some(2));
    }

    #[test]
    fn test_elements_from_counter() {
        let sample = [1, 2, 3, 4, 3, 3, 1, 2, 4, 5, 3, 2];

        let counter = Counter::new();
        let collection = Counter::from(counter, &sample);

        let elements = collection.elements().into_iter().collect::<Vec<_>>();

        assert!(!elements.is_empty()); // chech if it's not empty
    }

    #[test]
    fn test_most_common_counted() {
        let sample = [1, 2, 3, 4, 3, 3, 1, 2, 4, 5, 3, 2];

        let counter = Counter::new();
        let collection = Counter::from(counter, &sample);

        let most_common = collection.most_common(2);

        for (item, count) in most_common {
            print!("({item}, {count}), ");
        }
    }

    #[test]
    #[should_panic]
    fn test_most_common_counted_panic() {
        let sample = [1, 2];

        let counter = Counter::new();
        let collection = Counter::from(counter, &sample);

        // should panic because there are more things required then the actual number of unique items
        collection.most_common(3);
    }

    #[test]
    fn test_substract() {
        let sample_1 = [1, 2, 3, 4, 3, 3, 1, 2, 4, 5, 3, 2];
        let sample_2 = [1, 2, 3, 4, 3, 3, 1, 2, 4, 5, 3, 2];

        let counter_1 = Counter::new();
        let collection_1 = Counter::from(counter_1, &sample_1);

        let counter_2 = Counter::new();
        let collection_2 = Counter::from(counter_2, &sample_2);

        let res_counter = collection_1.substract(&collection_2);

        for (key, count) in res_counter.iter() {
            println!("Key: {key} has count: {count}");
        }
    }

    #[test]
    fn test_total_count() {
        let sample = [1, 2, 3, 4, 3, 3, 1, 2, 4, 5, 3, 2];

        let counter = Counter::new();
        let collection = Counter::from(counter, &sample);

        assert_eq!(collection.total(), 12);
    }

    #[test]
    fn test_counter_clear() {
        let sample = [1, 2, 3, 4, 3, 3, 1, 2, 4, 5, 3, 2];

        let counter = Counter::new();
        let mut collection = Counter::from(counter, &sample);

        collection.clear();

        assert!(collection.iter().collect::<Vec<_>>().is_empty());
    }
}
