mod animation;
mod bounds;
mod button;
mod combo;
mod component_wise;
mod draw_list;
mod font;
mod helper;
mod input;
mod input_color;
mod input_text;
mod map_ids;
mod popup;
mod slider;
mod spinner;
mod style;
mod text;
mod tree;
mod window;

pub use self::{
    animation::*, bounds::*, button::*, combo::*, component_wise::*, draw_list::*, font::*,
    helper::*, input::*, input_color::*, input_text::*, map_ids::*, popup::*, slider::*,
    spinner::*, style::*, text::*, tree::*, window::*,
};

pub type Point = [f32; 2];

pub type Rect = (Point, Point);
