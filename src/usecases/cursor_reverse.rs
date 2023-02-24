use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread};
use lazy_static::lazy_static;

lazy_static!{
    static ref CURSOR_REVERSE_ACTIVE: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

pub fn cursor_reverse_on(){
    let cursor_active = CURSOR_REVERSE_ACTIVE.clone();

    if cursor_active.load(Ordering::SeqCst){
        println!("Cursor reverse already working!");
        return;
    }

    println!("Starting cursor reverse...");

    thread::spawn(move ||{
        cursor_reverse();
    });

    cursor_active.store(true, Ordering::SeqCst);
}

pub fn cursor_reverse_off(){
    let cursor_active = CURSOR_REVERSE_ACTIVE.clone();

    if !cursor_active.load(Ordering::SeqCst){
        println!("Cursor reverse doesn't work anyway!");
        return;
    }

    println!("Stoping cursor reverse...");

    cursor_active.store(false, Ordering::SeqCst);
}

pub fn cursor_reverse_status() -> bool{
    CURSOR_REVERSE_ACTIVE.clone().load(Ordering::SeqCst)
}

fn cursor_reverse(){
    loop {
        if !CURSOR_REVERSE_ACTIVE.clone().load(Ordering::SeqCst){
            return;
        }
        // println!("Cursor reverse woking!");
    }
}