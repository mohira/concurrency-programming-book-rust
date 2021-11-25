struct Foo {
    val: u32,
}

fn main() {
    let mut x = Foo { val: 10 }; // x は mutable変数


    {
        let c = &x;
        println!("c.val = {}", c.val);
        println!("x.val = {}", x.val);

        let d = &mut x;
        d.val = 40;
        println!("d.val = {}", d.val);

        println!("c.val = {}", c.val);
    }

    println!("x.val = {}", x.val);
}