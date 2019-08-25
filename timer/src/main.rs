extern crate rodio;
use std::io::BufReader;
use std::time::Duration;
use std::{thread, time};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

struct Timer {
    pub time : u64,
}

fn run_timer(time : &mut Timer, rx : &Receiver<()>, tx : &Sender<u64>) {
    loop {
        thread::sleep( time::Duration::from_secs(1));
        if time.time == 0 {
            break;
        }
        time.time -= 1;
        if !rx.try_recv().is_err() {
            tx.send(time.time).unwrap();
        }
    }
}

fn sound_timer(time : &mut Timer) {
    loop {
        thread::sleep( time::Duration::from_secs(1));
        if time.time == 0 {
            break;
        }
        time.time -= 1;
    }

    let device = rodio::default_output_device().unwrap();

    let file = std::fs::File::open("beep.wav").unwrap();
    let beep1 = rodio::play_once(&device, BufReader::new(file)).unwrap();
    beep1.set_volume(0.2);
    thread::sleep(Duration::from_millis(100));
}

fn main() {
    let (time_sender, time_receiver) = mpsc::channel();
    let (notification_sender, notification_receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        let mut time = Timer{time: 10};
        run_timer(&mut time, &notification_receiver, &time_sender);
    });
    let handle2 = thread::spawn(move || {
        let mut time = Timer{time: 1};
        sound_timer(&mut time);
    });

    thread::sleep(time::Duration::from_secs(2));
    notification_sender.send(()).unwrap();
    let time = time_receiver.recv().unwrap();
    handle.join().unwrap();
    handle2.join().unwrap();
    println!("timer was: {}", time);
}
