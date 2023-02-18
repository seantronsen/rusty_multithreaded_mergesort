use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn mergesort(vector: &mut [u8]) -> Vec<u8> {
    let vector_length = vector.len();
    if vector_length < 2 {
        return vector.to_vec();
    }
    let pivot = vector_length / 2;
    let mut h1 = mergesort(&mut vector[..pivot]);
    let mut h2 = mergesort(&mut vector[pivot..]);

    merge(&mut h1[..], &mut h2[..])
}

fn merge(vec_a: &mut [u8], vec_b: &mut [u8]) -> Vec<u8> {
    let len_a = vec_a.len();
    let len_b = vec_b.len();
    let mut merged = Vec::with_capacity(len_a + len_b);
    let len_merged = len_a + len_b;
    let mut index_a = 0;
    let mut index_b = 0;

    while index_a + index_b < len_merged {
        if index_b == len_b || (index_a != len_a && vec_a[index_a] <= vec_b[index_b]) {
            merged.push(vec_a[index_a]);
            index_a += 1;
        } else if index_a == len_a || (index_b != len_b && vec_b[index_b] < vec_a[index_a]) {
            merged.push(vec_b[index_b]);
            index_b += 1;
        }
    }
    merged
}

/// Abstracted for consistent testing.
pub fn arrange_predetermined_random_sequence() -> Vec<u8> {
    let mut small_rng = StdRng::seed_from_u64(0_u64);
    let mut bytes = [0_u8; 10_000].to_vec();
    small_rng.fill(&mut bytes[..]);
    bytes
}

#[cfg(test)]
mod test_single_thread {

    use super::*;

    #[test]
    fn sort_ascending_order() {
        let mut bytes = arrange_predetermined_random_sequence();
        let result = mergesort(&mut bytes[..]);
        for i in 1..result.len() {
            assert!(result[i - 1] <= result[i]);
        }
    }
}

use std::sync::{Arc, Mutex};
use std::thread;

/// implementation will not handle mutex lock failures or thread failures for brevity.
pub fn mergesort_mt(
    collection: &mut [u8],
    max_threads: usize,
    thread_counter: Arc<Mutex<usize>>,
) -> Vec<u8> {
    let pivot = collection.len() / 2;
    let mut left = collection[..pivot].to_vec();
    let mut right = collection[pivot..].to_vec();
    let mut mutex_value = thread_counter.lock().unwrap();
    let num_remaining = max_threads - *mutex_value;
    if num_remaining >= 2 {
        *mutex_value += 2;
        drop(mutex_value);
        let counter = Arc::clone(&thread_counter);
        let max = max_threads;
        let handle_left = thread::spawn(move || mergesort_mt(&mut left[..], max, counter));
        let counter = Arc::clone(&thread_counter);
        let max = max_threads;
        let handle_right = thread::spawn(move || mergesort_mt(&mut right[..], max, counter));

        let mut left = handle_left.join().unwrap();
        let mut right = handle_right.join().unwrap();

        return merge(&mut left[..], &mut right[..]);
    }

    unimplemented!();
}

/*

multithreaded merge sort algorithm pseudo

fn mtms(collection, atomic_count, max_count) -> collection_sorted {
    let pivot = collection.len() // 2
    acquire atomic_count.lock()
    let num_remaining = max - atomic_count.value;
    if num_remaining >= 2 {
        atomic_count += 2;
        atomic_count.lock.release()
        handle_left = mtms(collection[..pivot], atomic_count, max_count)
        handle_right = mtms(collection[pivot..], atomic_count, max_count)
        return merge(handle_left.join(), handle_right.join())
    }
    else if num_remaining == 1 {
        atomic_count += 1;
        atomic_count.lock.release()
        handle_left = mtms(collection[..pivot], atomic_count, max_count)
        right = mtms(collection[pivot..], atomic_count, max_count)
        return merge(handle_left.join(), right)
    }
    else {
        atomic_count.lock.release()
        left = mtms(collection[..pivot], atomic_count, max_count)
        right = mtms(collection[pivot..], atomic_count, max_count)
    }
}


*/
