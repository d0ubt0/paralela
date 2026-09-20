use std::{thread, time::Instant};

fn main() {
    // 1. Generar array grande (100,000 elementos) sin librerías externas
    let n = 10000000;
    let mut s: Vec<i32> = Vec::with_capacity(n);
    let mut seed: u64 = 42;
    for _ in 0..n {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        s.push(((seed >> 33) as i32) % 500_000);
    }

    let k = 455_230;
    let q = 5;
    let x = 0.5;

    println!("--- PROBANDO CON {} ELEMENTOS (k = {}) ---", n, k);

    // 2. Solución real de referencia (usando el sort nativo de Rust)
    let start_ref = Instant::now();
    let mut reference = s.clone();
    reference.sort();
    let expected = reference[k];
    let dur_ref = start_ref.elapsed();
    println!(
        "Solución real (std sort) : {} | Tiempo: {:?}",
        expected, dur_ref
    );

    // 3. Prueba Secuencial
    let start_seq = Instant::now();
    let result_seq = sequential_select(s.clone(), q, k);
    let dur_seq = start_seq.elapsed();
    println!(
        "Secuencial               : {} | Tiempo: {:?}",
        result_seq, dur_seq
    );
    assert_eq!(
        result_seq, expected,
        "Error: El resultado secuencial no coincide"
    );

    // 4. Prueba Paralelo
    let start_par = Instant::now();
    let result_par = parallel_select(s.clone(), q, k, x);
    let dur_par = start_par.elapsed();
    println!(
        "Paralelo                 : {} | Tiempo: {:?}",
        result_par, dur_par
    );
    assert_eq!(
        result_par, expected,
        "Error: El resultado paralelo no coincide"
    );

    println!("\n¡Todos los resultados coincidieron con la solución real!");
}

fn sequential_select(mut array: Vec<i32>, q: usize, k: usize) -> i32 {
    if array.len() <= q {
        array.sort();
        return array[k];
    }

    let pivot = median_of_medians(&mut array, q);

    let mut s1 = Vec::new();
    let mut s2 = Vec::new();
    let mut s3 = Vec::new();

    for number in array {
        if number < pivot {
            s1.push(number);
        } else if number == pivot {
            s2.push(number);
        } else {
            s3.push(number);
        }
    }

    if k < s1.len() {
        sequential_select(s1, q, k)
    } else if k < s1.len() + s2.len() {
        pivot
    } else {
        sequential_select(s3, q, k - (s1.len() + s2.len()))
    }
}

fn median_of_medians(array: &mut [i32], q: usize) -> i32 {
    let len_array = array.len();

    if len_array <= q {
        array.sort();
        return array[len_array / 2];
    }

    let mut medians = Vec::new();

    for chunk in array.chunks_mut(q) {
        chunk.sort();
        medians.push(chunk[chunk.len() / 2]);
    }

    median_of_medians(&mut medians, q)
}

fn median_of_medians_parallel(mut array: Vec<i32>, q: usize, x: f32) -> i32 {
    let len_array = array.len();

    if len_array <= q {
        array.sort();
        return array[len_array / 2];
    }

    let available_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let np = ((len_array as f32).powf(1.0 - x).floor() as usize)
        .max(1)
        .min(available_threads);

    dbg!(np);
    let block_size = (len_array + np - 1) / np;

    let mut medians = Vec::with_capacity(np);

    thread::scope(|scope| {
        let mut threads = Vec::with_capacity(np);

        for chunk in array.chunks_mut(block_size) {
            let t = scope.spawn(move || {
                let mid = chunk.len() / 2;

                chunk.sort();

                chunk[mid]
            });

            threads.push(t);
        }

        for t in threads {
            medians.push(t.join().expect("Error en un hilo"));
        }
    });

    median_of_medians_parallel(medians, q, x)
}

fn parallel_select(mut array: Vec<i32>, q: usize, k: usize, x: f32) -> i32 {
    let array_len = array.len();

    if array_len <= q {
        array.sort();
        return array[k];
    }

    let pivot = median_of_medians_parallel(array.clone(), q, x);

    let mut s1 = Vec::new();
    let mut s2 = Vec::new();
    let mut s3 = Vec::new();

    for number in array {
        if number < pivot {
            s1.push(number);
        } else if number == pivot {
            s2.push(number);
        } else {
            s3.push(number);
        }
    }

    if k < s1.len() {
        parallel_select(s1, q, k, x)
    } else if k < s1.len() + s2.len() {
        pivot
    } else {
        parallel_select(s3.clone(), q, k - (s1.len() + s2.len()), x)
    }
}
