use std::thread;
fn main() {
    println!("Hello, world!");
    thread::spawn(f);
    thread::spawn(f);
    handle.join().unwrap();
}

fn f()  {
 
  println!("Hello from the thread!");
    let id=thread::current().id();
    println!("Thread id is: {:?}", id);

}