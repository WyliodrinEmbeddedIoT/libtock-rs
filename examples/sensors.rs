#![feature(alloc)]
#![no_std]

extern crate alloc;

use alloc::fmt::Write;
use tock::console::Console;
use tock::sensors::*;
use tock::timer;
use tock::timer::Duration;

fn main() {
    let mut console = Console::new();
    let mut humidity = HumiditySensor;
    let mut temperature = TemperatureSensor;
    let mut light = AmbientLightSensor;
    let mut ninedof = unsafe { Ninedof::new() };
    loop {
        write!(&mut console, "Humidity:    {}\n", humidity.read()).unwrap();
        write!(&mut console, "Temperature: {}\n", temperature.read()).unwrap();
        write!(&mut console, "Light:       {}\n", light.read()).unwrap();
        write!(
            &mut console,
            "Accel:       {}\n",
            ninedof.read_acceleration()
        )
        .unwrap();
        timer::sleep(Duration::from_ms(500));
    }
}
