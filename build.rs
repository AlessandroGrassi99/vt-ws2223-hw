fn main() {
    cc::Build::new().file("gen.c").compile("gen");
}
