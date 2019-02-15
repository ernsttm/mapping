# mapping
A rust based implementation of the DAGON timing based routing algorithm


# Compilation instructions

1. To compile this project, first install the rustup installation process, instructions can be found [here](https://www.rust-lang.org/tools/install).
2. After installing rustup, clone this git repository.
3. Then navigate to the cloned directory and execute "cargo build --release" to build an optimized release version.
    * This will generate an executable "<mapping_dir>/target/release/mapping"
    * Manually testing can be done by running ".../mapping <input_file>"
4. Testing
    * To test the basic first three inputs run cargo test --lib
    * To execute the last input file run cargo test -- --ignored
