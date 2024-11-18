use parallel_operations::parallel_binary_operation;
use std::time::{Duration, Instant};

fn main() {
    // Create a large dataset
    let mut seq_win = 0;
    let mut seq_total: Duration = Duration::new(0, 0);
    let mut par_win = 0;
    let mut par_total: Duration = Duration::new(0, 0);

    for _i in 0..10 {
        let large_data: Vec<i128> = (1..=10_000_000).collect(); // 10 million numbers

        // Measure time for parallel computation
        let start_parallel = Instant::now();
        let parallel_result = parallel_binary_operation(large_data.clone(), |a, b| a + b);
        let elapsed_parallel = start_parallel.elapsed();
        println!("Parallel computation took: {:?}", elapsed_parallel);
        println!("Parallel result: {}", parallel_result);

        // Measure time for sequential computation
        let start_sequential = Instant::now();
        let sequential_result: i128 = large_data.iter().copied().sum();
        let elapsed_sequential = start_sequential.elapsed();
        println!("Sequential computation took: {:?}", elapsed_sequential);
        println!("Sequential result: {}", sequential_result);

        // Compare times
        if elapsed_sequential > elapsed_parallel {
            println!(
                "Parallel computation is faster by: {:?}",
                elapsed_sequential - elapsed_parallel
            );
            println!(
                "Speedup: {}",
                elapsed_sequential.as_secs_f64() / elapsed_parallel.as_secs_f64()
            );
            par_win += 1;
        } else {
            println!(
                "Sequential computation is faster by: {:?}",
                elapsed_parallel - elapsed_sequential
            );
            println!(
                "Speedup: {}",
                elapsed_parallel.as_secs_f64() / elapsed_sequential.as_secs_f64()
            );
            seq_win += 1;
        }

        seq_total += elapsed_sequential;
        par_total += elapsed_parallel;

        // Optional: Compare results to ensure they are the same
        assert_eq!(
            parallel_result, sequential_result,
            "The results do not match!"
        );
    }

    // After all iterations, compare the average computation times
    let avg_seq_time = seq_total / 10; // Average sequential time
    let avg_par_time = par_total / 10; // Average parallel time

    println!("\n--- Final Results ---");
    if seq_win > par_win {
        println!(
            "Sequential computation won {} times out of {}",
            seq_win,
            seq_win + par_win
        );
        // Ensure no negative durations when calculating the difference
        let diff = if avg_seq_time > avg_par_time {
            avg_seq_time - avg_par_time
        } else {
            Duration::new(0, 0) // No negative durations
        };
        println!(
            "On average, sequential computation was faster by: {:?}",
            diff
        );
    } else {
        println!(
            "Parallel computation won {} times out of {}",
            par_win,
            seq_win + par_win
        );
        // Ensure no negative durations when calculating the difference
        let diff = if avg_par_time > avg_seq_time {
            avg_par_time - avg_seq_time
        } else {
            Duration::new(0, 0) // No negative durations
        };
        println!("On average, parallel computation was faster by: {:?}", diff);
    }

    // Display overall average speedup
    let speedup = if avg_par_time.as_secs_f64() > 0.0 {
        avg_seq_time.as_secs_f64() / avg_par_time.as_secs_f64()
    } else {
        avg_par_time.as_secs_f64() / avg_seq_time.as_secs_f64() // Avoid division by zero
    };

    println!("Average speedup: {:.2}x", speedup);
}
