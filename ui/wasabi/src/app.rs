use crate::{
    constants::{
        ADDRESS_BAR_HEIGHT, BLACK, DARK_GRAY, GRAY, LIGHT_GRAY, TITLE_BAR_HEIGHT, TOOLBAR_HEIGHT,
        WHITE, WINDOW_HEIGHT, WINDOW_INIT_X_POS, WINDOW_INIT_Y_POS, WINDOW_WIDTH,
    },
    cursor::Cursor,
};
use alloc::{
    format,
    rc::Rc,
    string::{String, ToString},
};
use core::cell::RefCell;
use noli::{
    error::Result as OsResult,
    prelude::SystemApi,
    println,
    rect::Rect,
    sys::{api::MouseEvent, wasabi::Api},
    window::{StringSize, Window},
};
use saba_core::{browser::Browser, error::Error, http::HttpResponse};

#[derive(PartialEq)]
enum InputMode {
    Normal,
    Editing,
}

pub struct WasabiUI {
    browser: Rc<RefCell<Browser>>,
    input_url: String,
    input_mode: InputMode,
    window: Window,
    cursor: Cursor,
}

impl WasabiUI {
    pub fn new(browser: Rc<RefCell<Browser>>) -> Self {
        Self {
            browser,
            input_url: String::new(),
            input_mode: InputMode::Normal,
            window: Window::new(
                "saba".to_string(),
                WHITE,
                WINDOW_INIT_X_POS,
                WINDOW_INIT_Y_POS,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
            )
            .unwrap(),
            cursor: Cursor::new(),
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

    fn handle_mouse_input(&mut self) -> Result<(), Error> {
        if let Some(MouseEvent { button, position }) = Api::get_mouse_cursor_info() {
            let relative_pos = (
                position.x - WINDOW_INIT_X_POS,
                position.y - WINDOW_INIT_Y_POS,
            );

            // outside window
            if relative_pos.0 < 0
                || relative_pos.0 > WINDOW_WIDTH
                || relative_pos.1 < 0
                || relative_pos.1 > WINDOW_HEIGHT
            {
                return Ok(());
            }

            self.window.flush_area(self.cursor.rect());
            self.cursor.set_position(position.x, position.y);
            self.window.flush_area(self.cursor.rect());
            self.cursor.flush();

            if button.l() || button.c() || button.r() {
                let relative_pos = (
                    position.x - WINDOW_INIT_X_POS,
                    position.y - WINDOW_INIT_Y_POS,
                );

                // click inside toolbar
                if relative_pos.1 >= TITLE_BAR_HEIGHT
                    && relative_pos.1 < TITLE_BAR_HEIGHT + TOOLBAR_HEIGHT
                {
                    self.clear_address_bar()?;
                    self.input_url = String::new();
                    self.input_mode = InputMode::Editing;
                    println!("button clicked in toolbar: {button:?} {position:?}");
                    return Ok(());
                }

                self.input_mode = InputMode::Normal;
            }
        }

        Ok(())
    }

    fn update_address_bar(&mut self) -> Result<(), Error> {
        if self
            .window
            .fill_rect(WHITE, 72, 4, WINDOW_WIDTH - 76, ADDRESS_BAR_HEIGHT - 2)
            .is_err()
        {
            return Err(Error::InvalidUI(
                "failed to clear an address bar".to_string(),
            ));
        }

        if self
            .window
            .draw_string(BLACK, 74, 6, &self.input_url, StringSize::Medium, false)
            .is_err()
        {
            return Err(Error::InvalidUI(
                "failed to update an address bar".to_string(),
            ));
        }

        self.window.flush_area(
            Rect::new(
                WINDOW_INIT_X_POS,
                WINDOW_INIT_Y_POS + TITLE_BAR_HEIGHT,
                WINDOW_WIDTH,
                TOOLBAR_HEIGHT,
            )
            .expect("failed to create a rect for the address bar"),
        );

        Ok(())
    }

    fn clear_address_bar(&mut self) -> Result<(), Error> {
        if self
            .window
            .fill_rect(WHITE, 72, 4, WINDOW_WIDTH - 76, ADDRESS_BAR_HEIGHT - 2)
            .is_err()
        {
            return Err(Error::InvalidUI(
                "failed to clear an address bar".to_string(),
            ));
        }

        self.window.flush_area(
            Rect::new(
                WINDOW_INIT_X_POS,
                WINDOW_INIT_Y_POS + TITLE_BAR_HEIGHT,
                WINDOW_WIDTH,
                TOOLBAR_HEIGHT,
            )
            .expect("failed to create a rect for the address bar"),
        );

        Ok(())
    }

    fn start_navigation(
        &mut self,
        handle_url: fn(String) -> Result<HttpResponse, Error>,
        destination: String,
    ) -> Result<(), Error> {
        match handle_url(destination) {
            Ok(response) => {
                let page = self.browser.borrow().current_page();
                page.borrow_mut().receive_response(response);
            }
            Err(e) => return Err(e),
        }

        Ok(())
    }

    fn handle_key_input(
        &mut self,
        handle_url: fn(String) -> Result<HttpResponse, Error>,
    ) -> Result<(), Error> {
        match self.input_mode {
            InputMode::Normal => {
                let _ = Api::read_key();
            }
            InputMode::Editing => {
                if let Some(c) = Api::read_key() {
                    // enter
                    if c == 0x0A as char {
                        self.start_navigation(handle_url, self.input_url.clone())?;
                        self.input_url = String::new();
                        self.input_mode = InputMode::Normal;
                    } else if c == 0x7F as char || c == 0x08 as char
                    // delete or backspace
                    {
                        self.input_url.pop();
                        self.update_address_bar()?;
                    } else {
                        self.input_url.push(c);
                        self.update_address_bar()?;
                    }
                }
            }
        }

        Ok(())
    }

    fn run_app(
        &mut self,
        handle_url: fn(String) -> Result<HttpResponse, Error>,
    ) -> Result<(), Error> {
        loop {
            self.handle_mouse_input()?;
            self.handle_key_input(handle_url)?;
        }
    }

    pub fn start(
        &mut self,
        handle_url: fn(String) -> Result<HttpResponse, Error>,
    ) -> Result<(), Error> {
        self.setup()?;
        self.run_app(handle_url)?;
        Ok(())
    }
}
