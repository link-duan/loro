use rle::HasLength;

use crate::{
    id::{Counter, ID},
    op::{utils::downcast_ref, Op},
    span::IdSpan,
    VersionVector,
};

use self::{
    content_map::ContentMap,
    cursor_map::{make_notify, CursorMap},
    y_span::{Status, YSpan},
};

use super::text_content::TextOpContent;

mod content_map;
mod cursor_map;
mod y_span;
mod yata;

/// A tracker for a single text, we can use it to calculate the effect of an operation on a text.
///
/// # Note
///
/// - [YSpan] never gets removed in both [ContentMap] and [CursorMap]
///     - The deleted contents are marked with deleted, but still lives on the [ContentMap] with length of 0
///
#[derive(Debug)]
struct Tracker {
    content: ContentMap,
    id_to_cursor: CursorMap,
}

impl From<ID> for u128 {
    fn from(id: ID) -> Self {
        ((id.client_id as u128) << 64) | id.counter as u128
    }
}

impl Tracker {
    pub fn new() -> Self {
        let min = ID::unknown(0);
        let max = ID::unknown(Counter::MAX);
        let len = (max.counter - min.counter) as usize;
        let mut content: ContentMap = Default::default();
        let mut id_to_cursor: CursorMap = Default::default();
        content.with_tree_mut(|tree| {
            tree.insert_notify(
                0,
                YSpan {
                    origin_left: None,
                    origin_right: None,
                    id: min,
                    status: Status::new(),
                    len,
                },
                &mut make_notify(&mut id_to_cursor),
            );
        });

        Tracker {
            content,
            id_to_cursor,
        }
    }

    fn turn_on(&mut self, _id: IdSpan) {}
    fn turn_off(&mut self, _id: IdSpan) {}
    fn checkout(&mut self, _vv: VersionVector) {}

    /// apply an operation directly to the current tracker
    fn apply(&mut self, op: &Op) {
        match &op.content {
            crate::op::OpContent::Normal { content } => {
                if let Some(text_content) = downcast_ref::<TextOpContent>(&**content) {
                    match text_content {
                        TextOpContent::Insert { id, text, pos } => {
                            self.content.insert_yspan_at_pos(
                                *id,
                                *pos,
                                text.len(),
                                &mut |_v, _leaf| {

                                    //TODO notify
                                },
                            );
                        }
                        TextOpContent::Delete {
                            id: _,
                            pos: _,
                            len: _,
                        } => todo!(),
                    }
                }
            }
            crate::op::OpContent::Undo { .. } => todo!(),
            crate::op::OpContent::Redo { .. } => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_turn_off() {
        let _tracker = Tracker::new();
        // tracker.turn_off(IdSpan::new(1, 1, 2));
    }
}
