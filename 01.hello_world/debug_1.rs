fn main() {
    #[allow(dead_code)]
    // This structure cannot be printed either with `fmt::Display` or
    // with `fmt::Debug`.
    struct UnPrintable(i32);
    //println!("UnPrintable: {:?}", UnPrintable(1)); // Error: `UnPrintable` cannot be formatted with the default formatter

    // The `derive` attribute automatically creates the implementation
    // required to make this `struct` printable with `fmt::Debug`.
    #[derive(Debug)]
    struct DebugPrintable(i32);
    println!("DebugPrintable: {:?}", DebugPrintable(1));
    println!("DebugPrintable: {:?}", DebugPrintable(2));
    println!("DebugPrintable: {:?}", DebugPrintable(3));
}
