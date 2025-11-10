use std::thread;
fn main() {
    let x = 1;
    let y = 2;
    let z = 3;
    //thread::scoped(|scope| { ... })：创建一个受限作用域，回调结束前，所有在该作用域里创建的线程都会被自动等待完成。这样可以安全地在闭包里借用外部局部变量，因为作用域保证借用不会在它们生命周期之外继续存在。
    let f = thread::scoped(|s| {
        s.spawn(move || {
            println!("x: {}", x);
            println!("y: {}", y);
            println!("z: {}", z);
        });
    });
    f.join().unwrap();
}
fn f(s: &Scope) {
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
}