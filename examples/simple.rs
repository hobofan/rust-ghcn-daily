extern crate ghcn_daily;

use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::fs::File;
use std::fs::create_dir_all;

use ghcn_daily::element::DlyLine;

// static INPUT_NAME: &'static str = "examples/GME00111445.dly.tail";
static INPUT_NAME: &'static str = "examples/GME00121150.dly.tail";

fn main() {
    let file = File::open(INPUT_NAME).unwrap();
    let reader = BufReader::new(&file);

    for line in reader.lines() {
        let l = line.unwrap();

        println!("{:?} {:?} {:?} - {:?} - {:?} {:?} {:?} {:?}",
                    DlyLine::station_id(&l),
                    DlyLine::year(&l),
                    DlyLine::month(&l),
                    DlyLine::element(&l).unwrap(),
                    DlyLine::value(&l, 1).unwrap(),
                    DlyLine::measurement(&l, 1).unwrap(),
                    DlyLine::quality(&l, 1).unwrap(),
                    DlyLine::source(&l, 1).unwrap(),
                );

        // println!("{} - {}", id, file_name);
    }
}
