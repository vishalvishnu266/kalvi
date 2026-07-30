pub mod button;
pub mod icon;
pub mod card;
pub mod input;
pub mod badge;
pub mod avatar;
pub mod stat;
pub mod select;
pub mod segmented;
pub mod tab_bar;
pub mod table;
pub mod list_item;
pub mod modal;
pub mod toast;
pub mod datepicker;
pub mod app_shell;
pub mod layout;
pub mod grid;
pub mod container;
pub mod storybook;

pub use button::Button;
pub use icon::Icon;
pub use card::Card;
pub use input::Input;
pub use badge::Badge;
pub use avatar::Avatar;
pub use stat::Stat;
pub use select::Select;
pub use segmented::Segmented;
pub use tab_bar::TabBar;
pub use table::Table;
pub use list_item::ListItem;
pub use modal::Modal;
pub use toast::Toast;
pub use datepicker::Datepicker;
pub use app_shell::AppShell;
pub use layout::{Stack, Row};
pub use grid::Grid;
pub use container::Container;

pub trait Render {
    fn render(&self) -> String;
}

impl<T: Render> Render for Vec<T> {
    fn render(&self) -> String {
        self.iter().map(|item| item.render()).collect::<Vec<_>>().join("\n")
    }
}
