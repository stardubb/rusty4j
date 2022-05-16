#[cfg(test)]
mod connector_tests {
    use crate::connection::connector::Connector;
    #[test]
    fn it_works() {
        let connector = Connector::new();
        assert_eq!(None, connector);
    }
}
