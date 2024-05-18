mod tests {
    use crate::cuid_wrapper;


    #[test]
    fn test_cuid() {
        let mut please_work = cuid_wrapper();
        println!("{}", please_work().unwrap())
    }
}
