/// 手柄连接测试
use rust_raspberry_car_controller::joystick::raw::js_event_c;
use std::fs::File;
use std::io::{BufReader, Read};
use std::mem;
use std::slice;

fn main() {
    let js_file = File::open("/dev/input/js0").unwrap();
    let mut rdr = BufReader::new(js_file);
    let config_size = mem::size_of::<js_event_c>();

    let mut config: js_event_c = unsafe { mem::zeroed() };
    loop {
        unsafe {
            let config_slice =
                slice::from_raw_parts_mut(&mut config as *mut _ as *mut u8, config_size);
            // `read_exact()` comes from `Read` impl for `&[u8]`
            rdr.read_exact(config_slice).unwrap();
        }

        println!("Read structure: {:#?}", config);
    }
}
