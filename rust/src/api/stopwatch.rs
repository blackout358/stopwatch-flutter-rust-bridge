use crate::frb_generated::StreamSink;
use anyhow::Result;
use flutter_rust_bridge::frb;
use std::thread;
use std::{thread::sleep, time::Duration};
use stopwatch::Stopwatch;
extern crate stopwatch;
use crossbeam::channel::{unbounded, Receiver, Sender};

const SLEEP_TIME: Duration = Duration::from_millis(100);

/*
Channels, stopwatch and remote are 2 seperate chunks. The front end only sees
the remote. Doesnt interact with stop watch

send stopwatch signal to do x y z.
'Can you please send me return signals every x time'

*/
// let mut buffer = Vec::<u8>::new();
// let buffer: Vec::<u8> = Vec::<u8>::new();

// const s = Semaphore::new(1);
#[derive(Debug)]

enum RemoteControl {
    Start,
    Stop,
    Time(String),
}
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
        let time_receiver = self.timer.stopwatch_to_remote.receiver.clone();
        loop {
            let _ = match time_receiver.try_recv() {
                Ok(message) => match message {
                    RemoteControl::Start => todo!(),
                    RemoteControl::Stop => todo!(),
                    RemoteControl::Time(data) => {
                        let mut parsed_data: i32 = data.parse().unwrap();
                        // println!("{}", &parsed_data / 10 * 10);
                        sink.add(parsed_data)
                    }
                },
                Err(_) => sink.add(-1),
            };
            thread::sleep(SLEEP_TIME);
        }
    }

    pub fn stop_timer(&self) {
        self.timer.stop_timer();
    }

    pub fn start_timer(&self) {
        self.timer.start_timer();
    }
}

#[frb(opaque)]
pub struct Timer {
    remote_to_stopwatch: ChannelPair,
    stopwatch_to_remote: ChannelPair,
}

impl Timer {
    #[frb(sync)]
    pub fn new() -> Self {
        let obj = Timer {
            remote_to_stopwatch: ChannelPair::new(),
            stopwatch_to_remote: ChannelPair::new(),
        };
        Self::main(&obj);

        obj
    }

    fn main(&self) {
        let stopwatch_receiver = self.remote_to_stopwatch.receiver.clone();
        let stopwatch_sender = self.stopwatch_to_remote.sender.clone();
        let _stopwatch_thread = thread::spawn(move || {
            let mut new_stopwatch = Stopwatch::new();
            loop {
                match stopwatch_receiver.try_recv() {
                    Ok(message) => match message {
                        RemoteControl::Start => {
                            println!("Start timer");
                            let _ = &new_stopwatch.start();
                        }
                        RemoteControl::Stop => {
                            println!("Stop timer");
                            let _ = &new_stopwatch.stop();
                        }
                        _ => {
                            println!("Doesnt match: {:?}", message);
                        }
                    },
                    Err(crossbeam::channel::TryRecvError::Empty) => println!("Nothing received"),
                    Err(crossbeam::channel::TryRecvError::Disconnected) => println!("Disconneced"),
                };
                let time = &new_stopwatch.elapsed_ms();
                let _res = stopwatch_sender.send(RemoteControl::Time(time.to_string()));
                println!("Time elapsed: {}\n", time);
                thread::sleep(SLEEP_TIME);
            }
        });
    }

    pub fn start_timer(&self) {
        let remote_sender = self.remote_to_stopwatch.sender.clone();
        let _result = remote_sender.send(RemoteControl::Start);
    }

    pub fn stop_timer(&self) {
        let remote_sender = self.remote_to_stopwatch.sender.clone();
        let _result = remote_sender.send(RemoteControl::Stop);
    }

    #[frb(sync)]
    pub fn return_something(&self) -> String {
        // self::tick(sink, timer);
        "Heyy from rust".to_string()
    }
}

#[frb(opaque)]
pub struct ChannelPair {
    sender: Sender<RemoteControl>,
    receiver: Receiver<RemoteControl>,
}

impl ChannelPair {
    fn new() -> Self {
        let (sender, receiver) = unbounded();
        ChannelPair { sender, receiver }
    }
}

const ONE_SECOND: Duration = Duration::from_secs(1);

pub fn reg_tick(sink: StreamSink<i32>) -> Result<()> {
    let mut ticks = 0;
    loop {
        let _result = sink.add(ticks);
        sleep(ONE_SECOND);
        if ticks == i32::MAX {
            break;
        }
        ticks += 1;
    }
    Ok(())
}
