extern crate firmata;
extern crate serial;

use firmata::*;
use serial::*;
use std::thread;

fn main() {
    let mut sp = serial::open("/dev/cu.usbmodemHIDF1").unwrap();

    sp.reconfigure(&|settings| {
        settings.set_baud_rate(Baud57600).unwrap();
        settings.set_char_size(Bits8);
        settings.set_parity(ParityNone);
        settings.set_stop_bits(Stop1);
        settings.set_flow_control(FlowNone);
        Ok(())
    }).unwrap();

    let mut b = firmata::Board::new(Box::new(sp)).unwrap();

    println!("firmware version {}", b.firmware_version());
    println!("firmware name {}", b.firmware_name());
    println!("protocol version {}", b.protocol_version());

    let mut enabled = false;

    loop {
        enabled = !enabled;
        println!("hid enabled: {}", enabled);
        if enabled {
            b.hid_enable();
        } else {
            b.hid_disable();
        }
        thread::sleep_ms(10_000);
    }
}
