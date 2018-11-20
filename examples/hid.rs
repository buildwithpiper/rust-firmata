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

    println!("ENABLED: {:?}", b.hid.enabled());
    println!("UP {:?}", b.hid.get_char(&HID_BUTTON_UP));
    println!("DOWN {:?}", b.hid.get_char(&HID_BUTTON_DOWN));
    println!("LEFT {:?}", b.hid.get_char(&HID_BUTTON_LEFT));
    println!("RIGHT {:?}", b.hid.get_char(&HID_BUTTON_RIGHT));
    println!("JOYSTICK {:?}", b.hid.get_char(&HID_BUTTON_JOYSTICK));

    let mut enabled;

    loop {
        enabled = b.hid.enabled().unwrap();
        println!("set enabled: {}", !enabled);
        if enabled {
            b.hid_set(HID_ENABLED, 0);
        } else {
            b.hid_set(HID_ENABLED, 1);
        }
        let mapping = b.hid.get_char(&HID_BUTTON_DOWN);
        println!("MAPPING {:?}", mapping);
        if mapping == Some('w') {
            b.hid_set(HID_BUTTON_DOWN, 'x' as u8);
        } else {
            b.hid_set(HID_BUTTON_DOWN, 'w' as u8);
        }
        thread::sleep_ms(10_000);
    }
}
