use neon::context::{Context, FunctionContext, ModuleContext};
use neon::handle::Handle;
use neon::object::Object;
use neon::result::{JsResult, NeonResult};
use neon::types::{JsArray, JsNumber, JsString, JsValue};
use rand::RngCore;
use sha2::{Digest, Sha256};

/// Calculates fibonacci of n
fn fibonacci(n: i32) -> i32 {
    // Recursive fibonacci
    match n {
        n if n < 1 => 0,
        n if n <= 2 => 1,
        _ => fibonacci(n - 1) + fibonacci(&n - 2),
    }
}

/// API to use fibonacci in Javascript
fn fibonacci_api(mut cx: FunctionContext) -> JsResult<JsNumber> {
    // Get param and convert it to Rust String type
    let string = cx.argument::<JsString>(0)?.value(&mut cx);

    // Parse String to i32
    // Throw Javascript TypeError if parse fails
    let number = match string.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return cx.throw_type_error(e.to_string()),
    };

    // Calculate fibonacci
    let res = fibonacci(number);

    // Convert result to Javascript number
    Ok(cx.number(res))
}

/// Multiply each element of a vector by 2
fn multiply_each_item_for_two(vector: Vec<f64>) -> Vec<f64> {
    // Multiply each element of a vector by 2 and return a new Vector
    vector.into_iter().map(|x| x * 2.).collect()
}

/// API to use multiply_each_item_for_two in Javascript
fn multiply_each_item_for_two_api(mut cx: FunctionContext) -> JsResult<JsArray> {
    // Get Javascript array param
    let js_arr_handle = cx.argument::<JsArray>(0)?;

    // Convert Javascript array to Rust Vector of JsValues
    let js_vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;

    // Convert every JsValue of the vector into f64
    // If conversion fails, throw Javascript TypeError
    let numbers: Vec<f64> = js_vec
        .into_iter()
        .map(
            |item| match item.downcast::<JsNumber, FunctionContext>(&mut cx) {
                Ok(n) => Ok(n.value(&mut cx)),
                Err(e) => cx.throw_type_error(e.to_string()),
            },
        )
        .flatten()
        .collect();

    // Creates a new Javascript array with a fixed length
    let js_array = JsArray::new(&mut cx, numbers.len() as u32);

    // Convert Rust vector into an enumerate to get indexes and values
    for (i, number) in multiply_each_item_for_two(numbers).into_iter().enumerate() {
        // Convert Rust f64 into Javascript Number
        let js_number = cx.number(number);

        // Push index and value to the Javascript array
        js_array.set(&mut cx, i as u32, js_number)?;
    }

    // Return Javascript array
    Ok(js_array)
}

/// Generate 100K Sha256 hashes with random bytes
fn generate_hashes_api(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let number = cx.argument::<JsNumber>(0)?.value(&mut cx);

    let mut all_data = vec![];

    for _ in 0..number as i64 {
        let mut data = [0u8; 18];
        rand::thread_rng().fill_bytes(&mut data);
        all_data.push(data);
    }

    for data in all_data.iter() {
        let mut hasher = Sha256::new();
        hasher.update(data);

        let result = hasher.finalize();

        let _hash = format!("{:x}", result);
    }

    Ok(cx.number(0))
}

/// Generate 1 Sha256 hash with a JS param
fn generate_hash(mut cx: FunctionContext) -> JsResult<JsString> {
    let data = cx.argument::<JsString>(0)?.value(&mut cx);

    let mut hasher = Sha256::new();
    hasher.update(data);

    Ok(cx.string(format!("{:x}", hasher.finalize())))
}

/// Export the functions to Javascript and rename them
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generate_hashes_rs", generate_hashes_api)?;
    cx.export_function("generate_hash_rs", generate_hash)?;
    cx.export_function("fibonacci_rs", fibonacci_api)?;
    cx.export_function(
        "multiply_each_item_for_two_rs",
        multiply_each_item_for_two_api,
    )?;

    Ok(())
}

#[cfg(test)]
mod fibonacci_tests {
    use crate::fibonacci;

    #[test]
    fn calculate_fibonacci_works() {
        let result = fibonacci(10);
        assert_eq!(result, 55);
    }
}

#[cfg(test)]
mod array_tests {
    use crate::multiply_each_item_for_two;

    #[test]
    fn calculate_multiply_each_item_for_two_works() {
        let numbers: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result: Vec<f64> = vec![2.0, 4.0, 6.0, 8.0, 10.0];

        assert_eq!(multiply_each_item_for_two(numbers), result);
    }
}
