pub struct GuiActions {
    pub quit: bool,
    pub clean_pressed: bool,
    pub dirty_pressed: bool,
}

impl GuiActions {
    pub fn should_continue(&self) -> bool {
        !self.quit
    }
}
