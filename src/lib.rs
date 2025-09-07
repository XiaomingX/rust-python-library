use pyo3::prelude::*;

/// Python可调用的加法函数
#[pyfunction]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// Python可调用的斐波那契数列生成函数
#[pyfunction]
fn fibonacci(n: usize) -> Vec<u64> {
    let mut sequence = vec![0, 1];
    if n <= 2 {
        return sequence[0..n].to_vec();
    }
    
    for i in 2..n {
        let next = sequence[i-1] + sequence[i-2];
        sequence.push(next);
    }
    sequence
}

/// 创建Python模块
#[pymodule]
fn rust_python_demo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}