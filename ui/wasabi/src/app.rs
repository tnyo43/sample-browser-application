use crate::constants::{
    ADDRESS_BAR_HEIGHT, BLACK, DARK_GRAY, GRAY, LIGHT_GRAY, TOOLBAR_HEIGHT, WHITE, WINDOW_HEIGHT,
    WINDOW_INIT_X_POS, WINDOW_INIT_Y_POS, WINDOW_WIDTH,
};
use alloc::{format, rc::Rc, string::ToString};
use core::cell::RefCell;
use noli::{
    error::Result as OsResult,
    prelude::SystemApi,
    println,
    sys::{api::MouseEvent, wasabi::Api},
    window::{StringSize, Window},
};
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

    fn setup_toolbar(&mut self) -> OsResult<()> {
        self.window
            .fill_rect(LIGHT_GRAY, 0, 0, WINDOW_WIDTH, TOOLBAR_HEIGHT)?;

        // border line
        self.window
            .draw_line(GRAY, 0, TOOLBAR_HEIGHT, WINDOW_WIDTH - 1, TOOLBAR_HEIGHT)?;
        self.window.draw_line(
            DARK_GRAY,
            0,
            TOOLBAR_HEIGHT + 1,
            WINDOW_WIDTH - 1,
            TOOLBAR_HEIGHT + 1,
        )?;

        // Text
        self.window
            .draw_string(BLACK, 5, 5, "Address:", StringSize::Medium, false)?;

        // Address bar
        self.window
            .fill_rect(WHITE, 70, 2, WINDOW_WIDTH - 74, ADDRESS_BAR_HEIGHT - 2)?;

        // Address bar vertical border
        self.window.draw_line(GRAY, 70, 2, WINDOW_WIDTH - 4, 2)?;
        self.window.draw_line(BLACK, 71, 3, WINDOW_WIDTH - 5, 3)?;
        // Address bar horizontal border
        self.window
            .draw_line(GRAY, 70, 2, 70, ADDRESS_BAR_HEIGHT - 2)?;
        self.window
            .draw_line(GRAY, 71, 3, 71, ADDRESS_BAR_HEIGHT - 1)?;

        Ok(())
    }

    fn setup(&mut self) -> Result<(), Error> {
        if let Err(error) = self.setup_toolbar() {
            return Err(Error::InvalidUI(format!(
                "failed to initialize a toolbar with error: {:#?}",
                error
            )));
        }

        self.window.flush();
        Ok(())
    }

    fn handle_mouse_input(&self) -> Result<(), Error> {
        if let Some(MouseEvent { button, position }) = Api::get_mouse_cursor_info() {
            println!("mouse position {:?}", position);
            if button.l() || button.c() || button.r() {
                println!("mouse clicked {:?}", button);
            }
        }

        Ok(())
    }

    fn run_app(&self) -> Result<(), Error> {
        loop {
            self.handle_mouse_input()?;
        }
    }

    pub fn start(&mut self) -> Result<(), Error> {
        self.setup()?;
        self.run_app()?;
        Ok(())
    }
}
