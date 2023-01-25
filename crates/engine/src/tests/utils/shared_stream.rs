use std::cell::RefCell;
use std::io;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct SharedStream(pub Rc<RefCell<Vec<u8>>>);

impl SharedStream {
    pub fn clear(&mut self) {
        (*self.0).borrow_mut().clear();
    }
}

impl io::Write for SharedStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (*self.0).borrow_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        (*self.0).borrow_mut().flush()
    }
}
