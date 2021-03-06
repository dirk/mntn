use std::borrow::Borrow;
use std::convert::AsRef;
use std::path::Path;
use std::sync::Arc;

use buffer::{Buffer, File};

#[derive(Clone, Debug, PartialEq)]
pub struct Pane {
    // Tabs inside a pane.
    pub items: Vec<Item>,
    pub active_item: Option<Item>,
}

impl Pane {
    pub fn new() -> Pane {
        Pane {
            items: vec![],
            active_item: None,
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item.clone());

        if self.active_item == None {
            self.active_item = Some(item);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Item {
    // As an optimization `Buffer`'s are managed separately and are
    // internally mutable.
    pub buffer: Arc<Buffer>,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        let self_buffer = self.buffer.borrow() as *const Buffer;
        let other_buffer = other.buffer.borrow() as *const Buffer;

        self_buffer == other_buffer
    }
}

impl Item {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Item {
        let buffer = Buffer::from_file(File::from_path(path.as_ref()));

        Item {
            buffer: Arc::new(buffer),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::Item;
    use super::super::buffer::Buffer;

    #[test]
    fn it_compares_pointers() {
        let buffer = Arc::new(Buffer::new_unknown());
        let item1 = Item { buffer: buffer.clone(), };
        let item2 = Item { buffer: buffer.clone(), };
        assert_eq!(item1, item2);

        let other_buffer = Arc::new(Buffer::new_unknown());
        let item3 = Item { buffer: other_buffer.clone() };
        assert_ne!(item1, item3);
    }
}
