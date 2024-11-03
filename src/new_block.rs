//!
//! New-type for Block.
//!
use crate::{CrossJoint, Joint};
use ratatui::widgets::Block;

/// New block.
#[derive(Debug, Default, Clone)]
pub struct NewBlock<'a> {
    pub block: Block<'a>,
    pub joints: Vec<Joint>,
    pub cross: Vec<CrossJoint>,
}
