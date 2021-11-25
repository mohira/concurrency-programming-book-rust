struct Foo {
    val: u32,
}

fn add_val(x: Foo, y: Foo) -> (u32, Foo, Foo) {
    (x.val + y.val, x, y) // 1
}

fn mul_val(x: Foo, y: Foo) -> (u32, Foo, Foo) {
    (x.val * y.val, x, y) // 2
}


fn main() {
    let x = Foo { val: 3 };
    let y = Foo { val: 6 };

    let (a, xn, yn) = add_val(x, y);
    // a = add_val(x,y)
    let (b, _, _) = mul_val(xn, yn);

    println!("a = {}, b = {}", a, b);
}