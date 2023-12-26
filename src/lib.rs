// Author: blackprince001
// Implementation of Python Counter Module
// https://docs.python.org/3/library/collections.html?highlight=counter#collections.Counter

use std::{collections::HashMap, panic};

#[derive(PartialEq, Eq, Clone)]
pub struct Counter<
    T: std::cmp::Eq + std::hash::Hash + std::marker::Copy + std::cmp::PartialOrd + std::cmp::PartialEq,
> {
    table: HashMap<T, usize>,
}

impl<
        T: std::cmp::Eq
            + std::hash::Hash
            + std::marker::Copy
            + std::cmp::PartialOrd
            + std::cmp::PartialEq,
    > Default for Counter<T>
{
    fn default() -> Self {
        Counter {
            table: HashMap::new(),
        }
    }
}

impl<
        T: std::cmp::Eq
            + std::hash::Hash
            + std::marker::Copy
            + std::cmp::PartialOrd
            + std::cmp::PartialEq,
    > Counter<T>
{
    pub fn new() -> Self {
        Counter {
            table: HashMap::new(),
        }
    }

    pub fn from(mut self, iterable: impl IntoIterator<Item = T> + std::clone::Clone) -> Counter<T> {
        for item in iterable.clone().into_iter() {
            if self.table.contains_key(&item) {
                *self.table.get_mut(&item).unwrap() += 1;
                continue;
            }
            self.table.insert(item, 1);
        }

        Counter { table: self.table }
    }

    pub fn get(&self, key: T) -> Option<usize> {
        if self.table.contains_key(&key) {
            self.table.get(&key).copied()
        } else {
            Some(0)
        }
    }

    pub fn update(&mut self, key: T, count: usize) {
        self.table.insert(key, count);
    }

    pub fn elements(&self) -> impl IntoIterator<Item = T> {
        let mut vector = vec![];
        for (item, count) in &self.table {
            for _ in 0..*count {
                vector.push(*item);
            }
        }

        vector.into_iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &usize)> {
        self.table.iter()
    }

    pub fn most_common(&self, n: usize) -> Vec<(&T, &usize)> {
        if n > self.table.keys().len() {
            panic!("{n} Exceeds the number of unique items in our Counter Struct.")
        }

        let mut iter = self.table.iter().collect::<Vec<_>>();
        let mut slice = vec![];

        // sort by descending order with counts from each
        iter.sort_by(|a, b| (b.1).partial_cmp(a.1).unwrap());
        iter[..n].clone_into(&mut slice);

        slice
    }

    pub fn substract(&self, other: &Self) -> Counter<T> {
        let mut ctmp = Counter::new();

        for (self_key, self_count) in &self.table {
            for (other_key, other_count) in &other.table {
                if self_key == other_key {
                    ctmp.update(
                        *self_key,
                        if self_count > other_count {
                            *self_count - *other_count
                        } else {
                            *other_count - *self_count
                        },
                    )
                }
            }
        }
        ctmp
    }

    pub fn total(&self) -> usize {
        let total_count: usize = self.table.values().sum();

        total_count
    }

    pub fn clear(&mut self) {
        self.table.clear()
    }
}

#[cfg(test)]
mod tests {

    use crate::Counter;

    #[test]
    fn test_string_counter() {
        let sample = ["bxffour", "bxffour", "blackprince"];

        let counter = Counter::new();
        let collection = Counter::from(counter, &sample);

        for (key, count) in collection.table {
            println!("Key: {key} has count: {count}");
        }
    }

    #[test]
    fn test_integer_counter() {
        let sample = [1, 2, 3, 4, 3, 3, 1, 2, 4, 5, 3, 2];

        let counter = Counter::new();
        let collection = Counter::from(counter, &sample);

        for (key, count) in collection.table {
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

        for (key, count) in res_counter.table {
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

        assert!(collection.table.is_empty());
    }
}
