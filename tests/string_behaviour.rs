#[test]
fn slice() {
    let a = "\"Minha string\" muito pika";
    let pos = a[1..].find('"').unwrap_or_else(|| 1);
    println!("{}|{}", &a[..pos], &a[pos..]);
}