use std::error::Error;
use std::fmt;

pub struct SimpleHandler;

impl eyre::EyreHandler for SimpleHandler {
    fn debug(&self, error: &(dyn Error + 'static), fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", error)?;

        Ok(())
    }
}

impl SimpleHandler {
    pub fn install() {
        let _ = eyre::set_hook(Box::new(|_| Box::new(SimpleHandler)));
    }
}
