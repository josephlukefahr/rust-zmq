#[test]
fn context_io_threads() {
    let ctx = zmq::Context::new();

    assert_eq!(
        ctx.get_io_threads().unwrap(),
        zmq::sys::ZMQ_IO_THREADS_DFLT as i32
    );

    ctx.set_io_threads(0).unwrap();
    assert_eq!(ctx.get_io_threads().unwrap(), 0);

    ctx.set_io_threads(7).unwrap();
    assert_eq!(ctx.get_io_threads().unwrap(), 7);

    assert!(ctx.set_io_threads(-1).is_err());
}
