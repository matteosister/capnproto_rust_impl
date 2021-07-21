use capnp::serialize_packed;
use capnproto::schema::person;
use clap::{App, Arg};
use std::fs::File;

fn main() {
    let matches = App::new("Cap 'N Proto Writer")
        .arg(
            Arg::with_name("output")
                .short("o")
                .value_name("FILE")
                .help("Output File")
                .takes_value(true),
        )
        .get_matches();

    let output_file = matches.value_of("output").unwrap();
    println!("Writing to {}", output_file);

    let mut message = capnp::message::Builder::new_default();

    let mut person = message.init_root::<person::Builder>();
    person.set_name("pippo");
    person.set_email("pippo@prima.it");

    let mut birthdate = person.reborrow().init_birthdate();
    birthdate.set_day(15);
    birthdate.set_month(11);
    birthdate.set_year(1979);

    let mut phone_numbers = person.reborrow().init_phones(2);

    let mut phone_number1 = phone_numbers.reborrow().get(0);
    phone_number1.set_number("123456");
    phone_number1.set_type(person::phone_number::Type::Home);

    phone_numbers.reborrow().get(1).set_number("999999");
    phone_numbers
        .reborrow()
        .get(1)
        .set_type(person::phone_number::Type::Work);

    let file = File::create(output_file).expect("unable to create file");

    serialize_packed::write_message(file, &mut message).unwrap();
}
