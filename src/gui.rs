use crate::Jot;

pub trait Labelable {
    fn short_label(&self, length: usize) -> String;
}
