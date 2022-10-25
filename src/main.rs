use std::sync::mpsc::channel;
use std::thread;
fn main() {
    // println!("Hello, world!");(())
    let (snd, rcv) = channel();
    let snd2 = snd.clone();

    thread::spawn(move || {
        snd.send("hello").unwrap();
    });

    thread::spawn(move || {
        snd2.send("world").unwrap();
    });

    let start = rcv.recv().unwrap();
    let end = rcv.recv().unwrap();

    let output = format!("{} {}!", start, end);

    println!("{output}");

}
