use std::slice::Chunks;

fn main() {
    let mut s = [8, -3, 5, -5, 6, 0, 10, 20].to_vec();
    let len_s = s.len();
    let k = 2;
    let q = 4;

    let mut s1 = Vec::new();
    let mut s2 = Vec::new();
    let mut s3 = Vec::new();

    let median_of_medians = median_of_medians(s.clone(), q);

    for n in s {
        if n < median_of_medians {
            s1.push(n);
        } else if n == median_of_medians {
            s2.push(n);
        } else {
            s3.push(n);
        }
    }

    print!("s");
    dbg!(s1);
    dbg!(s2);
    dbg!(s3);
}

fn median_of_medians(mut array: Vec<i32>, q: usize) -> i32 {
    let len_array = array.len();
    if len_array <= q {
        array.sort();
        return array[len_array / 2];
    } else {
        let mut medians = Vec::new();

        for chunk in array.chunks_mut(q) {
            chunk.sort();
            dbg!(&chunk);
            medians.push(chunk[chunk.len() / 2]);
        }

        return median_of_medians(medians, q);
    }
}
