extern crate ghcn_daily;

use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::fs::File;
use std::fs::create_dir_all;

use ghcn_daily::element::DlyLine;

static INPUT_NAME: &'static str = "examples/GME00111445.dly.tail";

fn main() {
    let file = File::open(INPUT_NAME).unwrap();
    let reader = BufReader::new(&file);

    for line in reader.lines() {
        let l = line.unwrap();
        // let line_length = l.len();
        // // only first and last lines are going to be one character long
        // if line_length == 1 {
        //     continue;
        // }
        println!("{:?} {:?} {:?} - {:?} {:?}",
                    DlyLine::station_id_raw(&l),
                    DlyLine::year_raw(&l),
                    DlyLine::month_raw(&l),
                    DlyLine::source(&l, 1),
                    DlyLine::value(&l, 1).unwrap());

        // println!("{} - {}", id, file_name);
    }
}
