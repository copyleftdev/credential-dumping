extern crate sysinfo;

use sysinfo::{System, SystemExt, ProcessExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    for process in sys.processes_by_name("ssh") {
        println!("{:?}", process.cmd());
    }
}
