fn main() {
    #[cfg(all(feature = "sha2", feature = "sha3"))]
    compile_error!("Please enable one of the SHA feature instead of enabling two of them.");

    #[cfg(not(feature = "sha2"))]
    {
        #[cfg(not(feature = "sha3"))]
        compile_error!("None of the feature has been enabled.")
    }
}