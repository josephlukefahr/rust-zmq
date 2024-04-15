use zmq::*;
use zmq::sys::errno;

#[test]
fn from_raw_eintr() {
    let error = Error::from_raw(errno::EINTR);
    assert_eq!(error, Error::EINTR);
}
