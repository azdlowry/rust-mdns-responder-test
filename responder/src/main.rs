extern crate env_logger;
extern crate mdns_responder;
extern crate dns_parser;

fn main() {
    
    env_logger::init();

    let responder = mdns_responder::Responder::new().unwrap();
    let _svc = responder.register(
        "_devpolterg._tcp".to_owned(),
        "deadbeefdeadbeef".to_owned(),
        8883,
        &["instance=dev", "id=deadbeefdeadbeef"]);

    loop {
        ::std::thread::sleep(::std::time::Duration::from_secs(10));
    }
}
