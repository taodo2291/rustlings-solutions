// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)


fn main() {
    let mut vec = Vec::new();

    fill_vec(&mut vec);

    println!("{} has length {} content `{:?}`", "vec1", vec.len(), vec);

    vec.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec.len(), vec);
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
