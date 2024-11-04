//!
//! New-type for Block.
//!
use crate::Joint;
use ratatui::widgets::{Block, BorderType};

/// Block extensions.
#[derive(Debug, Default, Clone)]
pub struct NewBlock<'a> {
    pub block: Block<'a>,

    /// Border type.
    ///
    /// The current Block impl discards this immediately, but would help greatly.
    /// Might be possible to extend BorderType with `Empty` and `Custom`.
    ///
    /// Creating a joint::Set ala border::Set is not doable.
    /// With the current parameterization this would need 192 fields.
    ///
    /// Thick border has a lot of glyphs, and Quadrants are surprisingly tricky.
    /// They are not fully covered yet.
    ///
    // pub border_type: BorderType,

    /// List of joints.
    pub joints: Vec<Joint>,
}
