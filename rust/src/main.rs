extern crate clap;

use clap::App;

fn main() {
    println!("{}: Hello world!", std::env::args().next().unwrap());

    let _ = App::new("foo").version("0.1.0").get_matches();
}
