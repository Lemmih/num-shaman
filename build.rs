use cc;

fn main() {
  cc::Build::new().file("src/erf.c").compile("foo");
}
