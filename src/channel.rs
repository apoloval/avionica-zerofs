use std::io;

pub trait PubChannel : io::Write {
    fn close(self) -> io::Result<()>;
}

pub mod nano {
    use std::io;

    use nanomsg;

    pub struct PubChannel {
        socket: nanomsg::Socket,
        endpoint: nanomsg::Endpoint,
    }

    impl io::Write for PubChannel {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> { self.socket.write(buf) }
        fn flush(&mut self) -> io::Result<()> { self.socket.flush() }
    }

    impl super::PubChannel for PubChannel {
        fn close(mut self) -> io::Result<()> {
            self.endpoint.shutdown().map_err(|e| io::Error::from(e))
        }
    }

    impl PubChannel {
        pub fn bind(addr: &str) -> nanomsg::Result<PubChannel> {
            let mut socket = nanomsg::Socket::new(nanomsg::Protocol::Pub)?;
            let endpoint = socket.bind(addr)?;
            Ok(PubChannel { socket, endpoint })
        }
    }
}
