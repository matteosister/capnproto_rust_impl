use capnproto::person::Person;
use capnproto::schema::person;
use clap::{App, Arg};
use std::convert::TryInto;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let matches = App::new("Cap 'N Proto Writer")
        .arg(
            Arg::with_name("input")
                .short("i")
                .value_name("FILE")
                .help("Input File")
                .takes_value(true),
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    println!("Reading from {}", input_file);

    let file = File::open(input_file).expect("unable to create file");

    let reader = capnp::serialize_packed::read_message(BufReader::new(file), Default::default())
        .expect("read error");
    let person_reader = reader
        .get_root::<person::Reader>()
        .expect("no person in data");

    let person: Person = person_reader.try_into().unwrap();
    dbg!(person);
}
