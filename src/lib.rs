#![allow(unused_imports)]
extern crate blake2;
extern crate byteorder;
extern crate claxon;
extern crate magic;
#[macro_use]
extern crate serde_derive;
extern crate simplemad;
extern crate smallvec;
extern crate time;

mod checksum;

use std::path::PathBuf;

pub fn check_songs() {
    let songs = vec![
        PathBuf::from("/home/bemeurer/src/musync/data/mono-sweep-1Hz-96KHz.flac"),
        PathBuf::from("/home/bemeurer/src/musync/data/stereo-sweep-1Hz-96KHz.flac"),
    ];

    for song in songs {
        let start = time::PreciseTime::now();
        let check = match checksum::check_file(&song) {
            Err(why) => panic!("{}", why),
            Ok(check) => check,
        };
        let end = time::PreciseTime::now();
        println!("{}", check);
        println!("---- {:?} took: {:?}", song, start.to(end));
    }
}
