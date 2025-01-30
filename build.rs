fn main() {
    // Specify the path to the directory containing the static library
    println!("cargo:rustc-link-search=native=C:\\Users\\hoeren\\Desktop\\Repos\\microsoft\\vcpkg\\installed\\x64-windows\\lib");
    
    // Specify the name of the library to link
    println!("cargo:rustc-link-lib=static=magic");
}