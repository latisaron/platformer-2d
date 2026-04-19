#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MenuItemType {
    Continue(String),
    Restart(String),
    Exit(String),
}

impl MenuItemType {
    pub fn text(&self) -> String {
        match self {
            MenuItemType::Continue(string) => {
                String::from(string.clone())
            },
            MenuItemType::Restart(string) => {
                String::from(string.clone())
            },
            MenuItemType::Exit(string) => {
                String::from(string.clone())
            },
        }
    }
}