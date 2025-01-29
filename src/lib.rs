use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
#[pyo3(name = "native_stress", signature = (threads=10, iterations=10, alloc_size=1024 * 1024))]
fn native_stress(threads: usize, iterations: usize, alloc_size: usize) -> PyResult<()> {
    use std::thread;

    // create threads
    let threads = (0..threads).map(|_| {
        thread::spawn(move || {
            for _ in 0..iterations {
                // allocate 1MB and free it using libc
                let ptr = unsafe { libc::malloc(alloc_size) };
                if !ptr.is_null() {
                    unsafe { libc::free(ptr) };
                }
            }
        })
    });

    for thread in threads {
        thread.join().unwrap();
    }

    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn malloc_stresser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(native_stress, m)?)?;
    Ok(())
}
