use rayon::prelude::*;

/// Gets the initial value for a binary operation.
///
/// This function determines the initial value based on the result of the operation
/// when applied to two sample values. It is used to determine the initial value
/// for parallel binary operations.
///
/// # Parameters
/// - `operation`: A closure that takes two operands of type `T` and returns a result of type `T`.
///
/// # Returns
/// The initial value for the binary operation based on the sample result.
/// For now either 0 or 1.
fn get_initial_value<T>(operation: fn(T, T) -> T) -> T
where
    T: Copy + Send + Sync + 'static + Default + PartialEq + From<u8>,
{
    let test_result = operation(T::from(8), T::from(8));
    match test_result {
        _ if test_result == T::from(16) => T::from(0), // For addition, use 0 as initial value
        _ if test_result == T::from(64) => T::from(1), // For multiplication, use 1 as initial value
        _ if test_result == T::from(0) => T::from(0),  // For subtraction, use 0 as initial value
        _ if test_result == T::from(1) => T::from(1),  // For division, use 1 as initial value
        _ => T::default(),                             // Default case
    }
}

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
pub fn parallel_binary_operation<T>(data: Vec<T>, operation: fn(T, T) -> T) -> T
where
    T: Copy + Send + Sync + 'static + Default + PartialEq + From<u8>,
{
    if data.is_empty() {
        return T::default();
    }
    if data.len() == 1 {
        return data[0];
    }

    let initial = get_initial_value(operation);

    let threads = num_cpus::get(); // Automatically use the number of available cores
    let chunk_size = (data.len() + threads - 1) / threads;

    // Perform the operation in parallel across chunks of data
    data.par_chunks(chunk_size)
        .map(|chunk| chunk.iter().copied().fold(initial, |a, b| operation(a, b)))
        .reduce(|| initial, |a, b| operation(a, b)) // Reduce results using operation
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

    // Test for multiplication operation
    #[test]
    fn test_parallel_multiplication() {
        let data = vec![1, 2, 3, 4, 5];
        let result = parallel_binary_operation(data, |a, b| a * b);
        assert_eq!(result, 120); // Expected result: 1 * 2 * 3 * 4 * 5 = 120
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
