struct Foo {
    val: u32,
}

fn main() {
    let mut x = Foo { val: 10 }; // x は mutable変数

    {
        let a = &mut x; // a は mutable参照
        println!("a.val = {}", a.val);

        // xは「&mut貸与中」のためエラー
        // println!("x.val = {}", x.val);

        let b: &Foo = a; // b は immutable参照
        a.val = 20;  // a は 「&貸与中」状態のためエラー

        println!("b.val = {}", b.val);
        // ここで、bが借用している所有権が返却される

        a.val = 30;
    }

    {
        let c = &x; // cはimmutable参照
        println!("c.val = {}", c.val);
        println!("x.val = {}", x.val);

        // let d = &mut x; // x は「&貸与中」状態のためのエラー
        // d.val = 40;

        println!("c.val = {}", c.val);
    }

    println!("x.val = {}", x.val);
}