use crate::frb_generated::StreamSink;
use anyhow::Result;
use flutter_rust_bridge::frb;
use std::cell::RefCell;
use std::fmt::format;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::{string, thread};
use std::{thread::sleep, time::Duration};
use stopwatch::Stopwatch;
extern crate stopwatch;
use async_lock::Semaphore;
const ONE_SECOND: Duration = Duration::from_secs(1);
use bytestream::*;
use crossbeam::channel::{bounded, unbounded, Receiver, Sender};
use std::io::{Cursor, Empty, Read, Result as IOResult, Write};
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
#[frb(opaque)]
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

    pub fn tick(&self, sink: StreamSink<i32>) -> Result<()> {
        let mut count = 0;
        // self.timer.tick(sink)
        let time_receiver = self.timer.stopwatch_to_remote.receiver.clone();
        loop {
            let _ = match time_receiver.try_recv() {
                Ok(message) => {
                    println!("{}", message);
                    sink.add(message.parse().unwrap())
                }
                Err(_) => sink.add(-1),
            };
            // count = count + 1;
            // sink.add(count);
            thread::sleep(Duration::from_millis(300))
        }
    }

    // pub fn tick(&self, sink: StreamSink<i32>) -> Result<()> {
    //     // timer.stop_watch.start();
    //     // self.stop_watch.start();

    //     let time_receiver = self.stopwatch_to_remote.receiver.clone();
    //     loop {
    //         let _ = match time_receiver.try_recv() {
    //             Ok(message) => {
    //                 println!("{}", message);
    //                 sink.add(message.parse().unwrap())
    //             }
    //             Err(_) => sink.add(1),
    //         };
    //         // println!("TIME: {:?}", result);
    //     }
    //     Ok(())
    // }

    pub fn stop_timer(&self) {
        self.timer.stop_timer();
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

        let _stopwatch_thread = thread::spawn(move || {
            let mut new_stopwatch = Stopwatch::new();

            // new_stopwatch.start();
            loop {
                // let mut watch = new_stopwatch.clone();
                // let temp = stopwatch_receiver.try_recv();
                println!("{}", new_stopwatch);
                match stopwatch_receiver.try_recv() {
                    Ok(message) => match message.as_str() {
                        "start" => {
                            println!("Start timer");
                            &new_stopwatch.start();
                        }
                        "stop" => {
                            println!("Stop timer");
                            &new_stopwatch.stop();
                        }
                        _ => {
                            println!("Doesnt match: {}", message);
                        }
                    },
                    Err(crossbeam::channel::TryRecvError::Empty) => println!("Nothing received"),
                    Err(crossbeam::channel::TryRecvError::Disconnected) => println!("Disconneced"),
                };
                let time = &new_stopwatch.elapsed_ms() / 1000;
                let _res = stopwatch_sender.send(time.to_string());
                println!("Time elapsed: {}\n", time);
                thread::sleep(Duration::from_millis(300));
                // new_stopwatch.start();
            }
        });

        // stopwatch_thread.join().unwrap();
    }

    // #[frb(sync)]
    pub fn start_timer(&self) {
        let remote_sender = self.remote_to_stopwatch.sender.clone();
        let _result = remote_sender.send(format!("start"));
    }

    // pub fn

    pub fn stop_timer(&self) {
        // *self.running.lock().unwrap() = false;
        let remote_sender = self.remote_to_stopwatch.sender.clone();

        // let _ = remote_sender.send(format!("Messag was sent")).unwrap();
        let _result = remote_sender.send(format!("stop"));
    }

    #[frb(sync)]
    pub fn return_something(&self) -> String {
        // self::tick(sink, timer);
        "Heyy from rust".to_string()
    }

    pub fn tick(&self, sink: StreamSink<i32>) -> Result<()> {
        // timer.stop_watch.start();
        // self.stop_watch.start();

        let time_receiver = self.stopwatch_to_remote.receiver.clone();
        loop {
            let _ = match time_receiver.try_recv() {
                Ok(message) => {
                    println!("{}", message);
                    sink.add(message.parse().unwrap())
                }
                Err(_) => sink.add(1),
            };
            // println!("TIME: {:?}", result);
        }
        // Ok(())
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
        let (sender, receiver) = unbounded(); // Bounded channel with a capacity of 100
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
