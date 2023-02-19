use rand::{rngs::StdRng, Rng, SeedableRng};
use std::thread;

pub fn mergesort(collection: &mut [u8]) -> Vec<u8> {
    let collection_length = collection.len();
    if collection_length < 2 {
        return collection.to_vec();
    }
    let pivot = collection_length / 2;
    let mut h1 = mergesort(&mut collection[..pivot]);
    let mut h2 = mergesort(&mut collection[pivot..]);

    merge(&mut h1[..], &mut h2[..])
}

fn merge(collection_a: &mut [u8], collection_b: &mut [u8]) -> Vec<u8> {
    let len_a = collection_a.len();
    let len_b = collection_b.len();
    let len_merged = len_a + len_b;
    let mut merged = vec![0u8; len_merged];
    let mut index_a = 0;
    let mut index_b = 0;

    while index_a + index_b < len_merged {
        if index_b == len_b || (index_a != len_a && collection_a[index_a] <= collection_b[index_b])
        {
            merged[index_a + index_b] = collection_a[index_a];
            index_a += 1;
        } else if index_a == len_a
            || (index_b != len_b && collection_b[index_b] < collection_a[index_a])
        {
            merged[index_a + index_b] = collection_b[index_b];
            index_b += 1;
        }
    }
    merged
}

pub fn arrange_deterministic_random_sequence(size: usize) -> Vec<u8> {
    let mut small_rng = StdRng::seed_from_u64(0_u64);
    let mut bytes = vec![0_u8; size];
    small_rng.fill(&mut bytes[..]);
    bytes
}

/// implementation will not handle thread failures for brevity.
pub fn mergesort_mt(collection: &mut [u8], max_threads: usize) -> Vec<u8> {
    if collection.len() < 2 {
        return collection.to_vec();
    }
    let pivot = collection.len() / 2;
    let (mut left, mut right) = collection.split_at_mut(pivot);
    let fork_a_count = max_threads / 2;
    let fork_b_count = max_threads - fork_a_count;
    if max_threads >= 2 {
        thread::scope(|scope| {
            let handle_left = scope.spawn(move || mergesort_mt(&mut left, fork_a_count));
            let handle_right = scope.spawn(move || mergesort_mt(&mut right, fork_b_count));
            let mut left = handle_left.join().unwrap();
            let mut right = handle_right.join().unwrap();
            return merge(&mut left[..], &mut right[..]);
        })
    } else {
        let mut left = mergesort_mt(&mut left[..], 0);
        let mut right = mergesort_mt(&mut right[..], 0);
        return merge(&mut left[..], &mut right[..]);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Abstracted for consistent testing.
    fn arrange_test_sequence() -> Vec<u8> {
        arrange_deterministic_random_sequence(10000)
    }
    fn assert_ascending(collection: &[u8]) {
        for i in 1..collection.len() {
            assert!(collection[i - 1] <= collection[i]);
        }
    }

    #[cfg(test)]
    mod single_threaded_tests {
        use super::*;

        #[test]
        fn sort_ascending_order() {
            let mut bytes = arrange_test_sequence();
            let result = mergesort(&mut bytes[..]);
            assert_ascending(&result)
        }
    }

    #[cfg(test)]
    mod mult_threaded_tests {
        use super::*;

        #[test]
        fn sort_ascending_order() {
            let mut bytes = arrange_test_sequence();
            let result = mergesort_mt(&mut bytes[..], 8);
            assert_ascending(&result)
        }
    }
}
