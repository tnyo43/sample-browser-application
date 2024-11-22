use noli::window::Window;
use saba_core::browser::Browser;

pub struct WasabiUI {
    browser: Rc<RefCell<Browser>>,
    window: Window,
}
