use rayon::prelude::*;

/// Performs a parallel binary operation on a vector of data.
///
/// This function divides the data into chunks, processes each chunk in parallel using
/// multiple threads, and combines the results using the provided binary operation.
///
/// # Parameters
/// - `data`: A vector of type `T` that contains the data to operate on.
/// - `operation`: A closure that takes two operands of type `T` and returns a result of type `T`.
///
/// # Returns
/// The result of applying the binary operation to all elements of the vector.
///
/// ```
pub fn parallel_binary_operation<T>(data: Vec<T>, operation: fn(T, T) -> T) -> T
where
    T: Copy + Send + Sync + 'static + Default,
{
    if data.is_empty() {
        return T::default();
    }

    let threads = num_cpus::get(); // Automatically use the number of available cores

    let chunk_size = (data.len() + threads - 1) / threads;

    // Perform the operation in parallel across chunks of data
    data.par_chunks(chunk_size)
        .map(|chunk| {
            chunk
                .iter()
                .copied()
                .fold(T::default(), |a, b| operation(a, b))
        })
        .reduce(|| T::default(), |a, b| operation(a, b)) // Reduce results using operation
}
#[cfg(test)]
mod tests {
    use super::*; // Import the public functions for testing

    // Test for addition operation
    #[test]
    fn test_parallel_addition() {
        let data = vec![1, 2, 3, 4, 5];
        let result = parallel_binary_operation(data, |a, b| a + b);
        assert_eq!(result, 15); // Expected result: 1 + 2 + 3 + 4 + 5 = 15
    }

    // Test for single element vector
    #[test]
    fn test_single_element() {
        let data = vec![42];
        let result = parallel_binary_operation(data, |a, b| a + b);
        assert_eq!(result, 42); // Only one element, should return that element
    }

    // Test for empty vector
    #[test]
    fn test_empty_vector() {
        let data: Vec<i32> = Vec::new();
        let result = parallel_binary_operation(data, |a, b| a + b);
        assert_eq!(result, 0); // Empty vector, result should be 0
    }

    // Test for odd number of elements (to check how chunking works)
    #[test]
    fn test_odd_number_of_elements() {
        let data = vec![1, 2, 3, 4, 5];
        let result = parallel_binary_operation(data, |a, b| a + b);
        assert_eq!(result, 15); // 1 + 2 + 3 + 4 + 5 = 15
    }
}


