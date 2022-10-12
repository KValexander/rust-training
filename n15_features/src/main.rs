/* Struct Rectangle */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/* Entry point */
fn main() {

    /* List rectangles */
    let mut list = [
        Rectangle {width: 10, height: 1},
        Rectangle {width: 3, height: 5},
        Rectangle {width: 7, height: 12},
    ];

    /* Sort list */
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    /* Out list */
    println!("{:#?}, sorted {}", list, num_sort_operations);

    /* Iterator */
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }


}