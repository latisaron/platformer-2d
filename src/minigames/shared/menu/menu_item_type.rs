#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MenuItemType {
    Continue,
    Restart,
    Exit,
}

impl MenuItemType {
    pub fn text(&self) -> String {
        match self {
            MenuItemType::Continue => {
                String::from("Continue")
            },
            MenuItemType::Restart => {
                String::from("Restart")
            },
            MenuItemType::Exit => {
                String::from("Exit")
            },
        }
    }
}