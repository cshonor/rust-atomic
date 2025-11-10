use std::thread;
fn main() {
    let mut v = vec![1, 2, 3];
    let x = 1;
    let y = 2;
    let z = 3;
    //thread::scoped(|scope| { ... })：
    // 创建一个受限作用域，回调结束前，
    // 所有在该作用域里创建的线程都会被自动等待完成。
    // 这样可以安全地在闭包里借用外部局部变量，
    // 因为作用域保证借用不会在它们生命周期之外继续存在。
    thread::scoped(|s| {
        //scope.spawn(|scope_thread| { ... })（
        // 旧写法是你代码里的 s.spawn）：
        // 在该作用域内启动一个新线程。这里可以直接捕获 x、y、z 等外部变量而无需 move，闭包运行完毕后会自动 join，避免遗漏加入。
        s.spawn(move || {
            println!("x: {}", x);
            println!("y: {}", y);
            println!("z: {}", z);
        })
        s.spawn(move || {
            println!("v: {:?}", v);
        })//scope.spawn 启动的线程没有先后保证，哪一个先运行、先打印完全由操作系统调度决定。同一个程序每次执行，输出顺序都可能不同。如果需要确定顺序，必须显式同步，比如在一个线程里等待另一个线程完成后再继续。
        ;
    });
//新版 thread::scope 则通常依赖作用域结束时自动等待所有子线程完成，
// 避免了显式 join 的遗漏问题，更符合 Rust 的惯用写法。
}
