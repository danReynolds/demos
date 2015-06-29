extern crate gcc;

fn main() {
    gcc::Config::new().file("src/add_one.c").compile("libadd.a");
}
