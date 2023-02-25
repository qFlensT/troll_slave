use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread};
use lazy_static::lazy_static;
use windows::Win32::{Foundation::POINT, UI::WindowsAndMessaging::{GetCursorPos, SetCursorPos, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN}};

lazy_static!{
    static ref CURSOR_REVERSE_ACTIVE: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

pub fn cursor_reverse_on(){
    let cursor_active = CURSOR_REVERSE_ACTIVE.clone();

    if cursor_active.load(Ordering::SeqCst){
        return;
    }

    thread::spawn(||{
        cursor_reverse();
    });

    cursor_active.store(true, Ordering::SeqCst);
}

pub fn cursor_reverse_off(){
    let cursor_active = CURSOR_REVERSE_ACTIVE.clone();

    if !cursor_active.load(Ordering::SeqCst){
        return;
    }

    cursor_active.store(false, Ordering::SeqCst);
}

pub fn cursor_reverse_status() -> bool{
    CURSOR_REVERSE_ACTIVE.clone().load(Ordering::SeqCst)
}

pub fn cursor_reverse(){
    unsafe fn screen_res(point: &mut POINT){
        point.x = GetSystemMetrics(SM_CXSCREEN );
        point.y = GetSystemMetrics(SM_CYSCREEN );
    }

    let mut screen = POINT::default();

    unsafe{
        screen_res(&mut screen);
    }
    
    loop {
        if !CURSOR_REVERSE_ACTIVE.clone().load(Ordering::SeqCst){
            return;
        }

        let mut point0 = POINT::default();
        unsafe{
            GetCursorPos(&mut point0);
        }

        let calibrate_cordx;
        let calibrate_cordy;

        if point0.x >= (screen.x - 5){
            calibrate_cordx = screen.x - 5;
        }else if point0.x < 5{
            calibrate_cordx = 5;
        }else{
            calibrate_cordx = point0.x
        }

        if point0.y >= (screen.y - 5){
            calibrate_cordy = screen.y - 5;
        }else if point0.y < 5{
            calibrate_cordy = 5;
        }else{
            calibrate_cordy = point0.y
        }

        unsafe{
            SetCursorPos(calibrate_cordx, calibrate_cordy);
        }

        let mut point1 = POINT::default();
        unsafe{
            GetCursorPos(&mut point1);
        }

        if point0 != point1{
            let dx = point1.x - point0.x;
            let dy = point1.y - point0.y;

            let mut x: i32;
            let mut y: i32;

            if dx.is_negative(){
                x = point0.x + dx.abs();
            }else{
                x = point0.x - dx;
            }

            if dy.is_negative(){
                y = point0.y + dy.abs();
            }else{
                y = point0.y - dy;
            }

            if x >= screen.x{
                x = screen.x - 5;
            }else if x < 5 {
                x = 5;
            }

            if y >= screen.y{
                y = screen.y - 5;
            }else if y < 5 {
                y = 5;
            }

            unsafe{SetCursorPos(x, y);}
        }
    }
}