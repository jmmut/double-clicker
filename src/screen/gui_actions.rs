
pub struct GuiActions {
    pub quit: bool,
}

impl GuiActions {
    pub fn should_continue(&self) -> bool {
        !self.quit
    }
}
