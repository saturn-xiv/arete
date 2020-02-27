extern crate arete;
extern crate env_logger;

struct Handler;

impl arete::queue::mosquitto::Handler for Handler {
    type Item = String;
    type Error = arete::errors::Error;
    fn handle(&self, payload: &Self::Item) -> Result<(), Self::Error> {
        println!("{}", payload);
        Ok(())
    }
}

#[test]
fn it_sub() {
    env_logger::init();
    let mut cli = arete::queue::mosquitto::Connection::new("localhost", None).unwrap();
    cli.receive("unittest", &vec!["hi".to_string()], &Handler)
        .unwrap();
}
