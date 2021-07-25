use std::{sync::mpsc::{self, Receiver, Sender}, thread::{self, JoinHandle}, time::{Duration, Instant}};

pub struct DelayTimer {
    value: u8,

    work_handle: Option<JoinHandle<()>>,
    work_send_stop: Sender<bool>,
    work_send_update: Sender<u8>,

    work_receive_value: Receiver<u8>,
}

impl DelayTimer {
    pub fn new(wait_between_cycles: Duration) -> DelayTimer {
        let (work_send_stop, work_receive_stop): (Sender<bool>, Receiver<bool>) = mpsc::channel();
        let (work_send_update, work_receive_update): (Sender<u8>, Receiver<u8>) = mpsc::channel();
        let (work_send_value, work_receive_value): (Sender<u8>, Receiver<u8>) = mpsc::channel();
        
        let value: u8 = 0;
        DelayTimer {
            value: value.clone(),
            work_handle: Some(thread::spawn(move || {
                            DelayTimer::update(value.clone(), wait_between_cycles, work_receive_stop, work_receive_update, work_send_value)
                        })),
            work_send_stop,
            work_send_update,
            work_receive_value
        }
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }

    fn update(
        start_value: u8,
        wait_between_cycles: Duration, 
        work_receive_stop: Receiver<bool>,
        work_receive_update: Receiver<u8>,
        work_send_value: Sender<u8>,
    ) {
        let mut value: u8 = start_value;
        let mut last_update = Instant::now();

        loop {
            match work_receive_stop.try_recv() {
                Ok(stop) => {
                    if stop { break; }
                },
                Err(error) => {
                    println!("While receiving updates {}", error);
                    // TODO: report up
                    return;
                }
            }

            let previous_value = value;

            for update in work_receive_update.try_iter() {
                value = update;
            }
            
            if last_update.elapsed() >= wait_between_cycles && value > 0 {
                value -= 1;
                last_update = Instant::now();
            }

            if value != previous_value {
                match work_send_value.send(value) {
                    Ok(_) => {},
                    Err(error) => { 
                        // TODO: report up
                        println!("While sending value {}", error);
                    }
                }
            }
        }
    }
}

impl Drop for DelayTimer {
    fn drop(&mut self) {
        match self.work_send_stop.send(true) {
            Ok(_) => {},
            Err(error) => { 
                println!("While sending drop {}", error);
                // TODO: report up
            }
        }
        
        match self.work_handle.take() {
            Some(thread) => {
                match thread.join() {
                    Ok(_) => {},
                    Err(result) => { 
                        println!("Error while drop joining");
                        // TODO: report up
                    }
                }
            },
            None => {
                // TODO: report up
            }
        }
    }
}

impl chip8_traits::Timer for DelayTimer {
    // TODO: should update be in an update function so as one would expect ::value doesn't have to be self mutable?
    fn get(&mut self) -> u8 {
        // Does this mean out channel can grow stupidly long if set often but don't check as often?
        for update in self.work_receive_value.iter() {
            self.value = update;
        }
        self.value
    }

    fn set(&mut self, value: u8) {
        self.value = value;
        self.work_send_update.send(value);
    }
}