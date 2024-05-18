mod tests {
    use crate::Cuid;

    #[test]
    fn test_cuid() {
        println!("{}",Cuid::default().generate(None).unwrap())
    }
}