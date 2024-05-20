use crate::frb_generated::StreamSink;
use anyhow::Result;
use flutter_rust_bridge::frb;
use std::cell::RefCell;
use std::fmt::format;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{thread::sleep, time::Duration};
use stopwatch::Stopwatch;
extern crate stopwatch;
use async_lock::Semaphore;
const ONE_SECOND: Duration = Duration::from_secs(1);
use bytestream::*;
use crossbeam::channel::{bounded, Receiver, Sender};
use std::io::{Cursor, Read, Result as IOResult, Write};
use std::sync::mpsc;

/*
Channels, stopwatch and remote are 2 seperate chunks. The front end only sees
the remote. Doesnt interact with stop watch

send stopwatch signal to do x y z.
'Can you please send me return signals every x time'

*/
// let mut buffer = Vec::<u8>::new();
// let buffer: Vec::<u8> = Vec::<u8>::new();

// const s = Semaphore::new(1);
// #[derive(Debug)]
#[frb(non_opaque)]
pub struct StopwatchRemote {
    pub timer: Timer,
}

impl StopwatchRemote {
    #[frb(sync)]
    pub fn new() -> Self {
        StopwatchRemote {
            timer: Timer::new(),
        }
    }

    pub fn start_timer(&self) {
        self.timer.start_timer();
    }
}

#[frb(opaque)]
pub struct Timer {
    stop_watch: Stopwatch,
    running: bool,
    mutex: Mutex<()>,
    buffer: Vec<u8>,
    remote_to_stopwatch: ChannelPair,
    stopwatch_to_remote: ChannelPair,
}

impl Timer {
    #[frb(sync)]
    pub fn new() -> Self {
        let obj = Timer {
            stop_watch: Stopwatch::new(),
            // stop_watch: Stopwatch::new(),
            running: false,
            mutex: Mutex::new(()),
            buffer: Vec::<u8>::new(),
            remote_to_stopwatch: ChannelPair::new(),
            stopwatch_to_remote: ChannelPair::new(),
        };
        Self::main(&obj);

        obj
    }

    fn main(&self) {
        // let remote_sender = self.remote_to_stopwatch.sender.clone();
        let stopwatch_receiver = self.remote_to_stopwatch.receiver.clone();
        let stopwatch_sender = self.stopwatch_to_remote.sender.clone();
        // let remote_receiver = self.

        let stopwatch_thread = thread::spawn(move || {
            let new_stopwatch = Stopwatch::new();

            loop {
                match stopwatch_receiver.try_recv() {
                    Ok(message) => println!("Message: {}", message),
                    Err(crossbeam::channel::TryRecvError::Empty) => println!("Nothing received"),
                    Err(crossbeam::channel::TryRecvError::Disconnected) => println!("Disconnected"),
                }

                let _ = stopwatch_sender.send((new_stopwatch.elapsed_ms() / 1000).to_string());
                thread::sleep(Duration::from_secs(1));
            }
        });

        stopwatch_thread.join().unwrap();
    }

    // #[frb(sync)]
    pub fn start_timer(&self) {
        let remote_sender = self.remote_to_stopwatch.sender.clone();
        let _ = remote_sender.send(format!("Messag was sent"));
    }

    // pub fn

    pub fn stop_timer(&mut self) {
        let _guard = self.mutex.lock().unwrap();
        // *self.running.lock().unwrap() = false;
        self.stop_watch.stop();
    }

    #[frb(sync)]
    pub fn return_something(&self) -> String {
        // self::tick(sink, timer);
        "Heyy from rust".to_string()
    }

    pub fn tick(self, sink: StreamSink<i32>) -> Result<()> {
        // timer.stop_watch.start();
        // self.stop_watch.start();

        loop {
            let value_to_add = { Self::get_time_elapsed(&self) };

            let _ = sink.add(value_to_add);
            sleep(ONE_SECOND);

            // println!("TIME: {:?}", result);
        }
        Ok(())
    }

    pub fn get_time_elapsed(&self) -> i32 {
        let _guard = self.mutex.lock().unwrap();
        (self.stop_watch.elapsed_ms() / 1000) as i32
    }

    // fn

    pub async fn send_off(&self, sink: StreamSink<i32>) -> Result<()> {
        // timer.stop_watch.start();
        // self.stop_watch.start();
        loop {
            // let value_to_add = {
            //     let _guard = self.mutex.lock().unwrap();
            //     (self.stop_watch.elapsed_ms() / 1000) as i32
            // };

            // let _ = sink.add(value_to_add);
            // sleep(ONE_SECOND);

            // println!("TIME: {:?}", result);
        }
        Ok(())
    }
}

#[frb(opaque)]
pub struct ChannelPair {
    pub sender: Sender<String>,
    pub receiver: Receiver<String>,
}

impl ChannelPair {
    fn new() -> Self {
        let (sender, receiver) = bounded(100); // Bounded channel with a capacity of 100
        ChannelPair { sender, receiver }
    }
}
// const  local_timer = MyTimer::new("passed_name");

#[flutter_rust_bridge::frb(sync)]
pub fn start_timer(timer: &mut Timer) {
    timer.stop_watch.start();
}

pub fn tick(sink: StreamSink<i32>, timer: &Timer) -> Result<()> {
    // timer.stop_watch.start();
    loop {
        let result = sink.add((timer.stop_watch.elapsed_ms() / 1000) as i32);
        sleep(ONE_SECOND);
        // println!("TIME: {:?}", result);
    }
    Ok(())
}

pub fn reg_tick(sink: StreamSink<i32>) -> Result<()> {
    let mut ticks = 0;
    loop {
        sink.add(ticks);
        sleep(ONE_SECOND);
        if ticks == i32::MAX {
            break;
        }
        ticks += 1;
    }
    Ok(())
}
