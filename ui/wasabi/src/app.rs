use crate::constants::{WHITE, WINDOW_HEIGHT, WINDOW_INIT_X_POS, WINDOW_INIT_Y_POS, WINDOW_WIDTH};
use alloc::{format, rc::Rc, string::ToString};
use core::cell::RefCell;
use noli::{error::Result as OsResult, window::Window};
use saba_core::{browser::Browser, error::Error};

pub struct WasabiUI {
    browser: Rc<RefCell<Browser>>,
    window: Window,
}

impl WasabiUI {
    pub fn new(browser: Rc<RefCell<Browser>>) -> Self {
        Self {
            browser,
            window: Window::new(
                "saba".to_string(),
                WHITE,
                WINDOW_INIT_X_POS,
                WINDOW_INIT_Y_POS,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
            )
            .unwrap(),
        }
    }

    fn setup(&mut self) -> Result<(), Error> {
        self.window.flush();
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), Error> {
        self.setup()?;
        Ok(())
    }
}
