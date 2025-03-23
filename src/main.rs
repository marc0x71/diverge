use distance::edit_distance;

#[macro_use]
mod macros;
mod distance;

fn main() {
    let s1 = ['m', 'a', 'r', 'c', 'o'];
    let s2 = ['m', 'a', 'r', 'i', 'o'];

    let distances = edit_distance(&s1, &s2);
    dbg!(s1, s2, distances);
}
