//! # Full Model
//!
//! Combining Editor and Cuestack to pass through functions easier.

use crate::model::cuestack::CueStack;
use crate::model::editor::Editor;

/// Combined Model to reference
///
/// Includes an Editor, as well as a Cuestack
pub struct Model {
    editor: Editor,
    cuestack: CueStack,
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

impl Model {
    /// Creates a new model
    ///
    /// This creates a new `Editor`, as well as a new `CueStack`. In most cases this should be mutable
    ///
    /// # Examples
    ///
    /// ```
    /// use cues::model::full_model::Model;
    ///
    /// let model = Model::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        let editor = Editor::new();
        let cuestack = CueStack::new();
        Self { editor, cuestack }
    }

    /// Returns the Model's Editor (Immutable)
    ///
    /// This is an immutable reference, use `editor_mut()` for an
    /// immutable reference.
    /// Note that there is no setter function as you should never need to set a new editor.
    ///
    /// # Examples
    ///
    /// ```
    /// use cues::model::full_model::Model;
    ///
    /// let model = Model::new();
    /// let editor = model.editor();
    /// ```
    #[must_use]
    pub const fn editor(&self) -> &Editor {
        &self.editor
    }

    /// Returns the Model's Editor (Mutable)
    ///
    /// This is a mutable reference, use `editor()` for a
    /// mutable reference.
    /// Note that there is no setter function as you should never need to set a new editor.
    ///
    /// # Examples
    ///
    /// ```
    /// use cues::model::full_model::Model;
    ///
    /// let mut model = Model::new();
    /// let editor = model.editor_mut();
    /// ```
    #[must_use]
    pub const fn editor_mut(&mut self) -> &mut Editor {
        &mut self.editor
    }

    /// Returns the Model's Cuestack (Immutable)
    ///
    /// This is an immutable reference, use `cuestack_mut()`
    /// for a mutable reference
    ///
    /// # Examples
    ///
    /// ```
    /// use cues::model::full_model::Model;
    ///
    /// let model = Model::new();
    /// let editor = model.cuestack();
    /// ```
    #[must_use]
    pub const fn cuestack(&self) -> &CueStack {
        &self.cuestack
    }

    /// Returns the Model's Cuestack (Mutable)
    ///
    /// This is a mutable reference, use `cuestack()`
    /// for an immutable reference
    ///
    /// # Examples
    ///
    /// ```
    /// use cues::model::full_model::Model;
    ///
    /// let mut model = Model::new();
    /// let editor = model.cuestack_mut();
    /// ```
    #[must_use]
    pub const fn cuestack_mut(&mut self) -> &mut CueStack {
        &mut self.cuestack
    }

    /// Sets a new Cuestack for the model
    ///
    /// This currently has no purposes, but for multiple
    /// cuestacks in the future it may be helpful.
    ///
    /// # Examples
    ///
    /// ```
    /// use cues::model::{full_model::Model, cuestack::CueStack};
    ///
    /// let mut model = Model::new();
    /// let new_cuestack = CueStack::new();
    /// let editor = model.set_cuestack(new_cuestack);
    /// ```
    pub fn set_cuestack(&mut self, cuestack: CueStack) {
        self.cuestack = cuestack;
    }

    /// Returns a mutable reference to both the Editor and Cuestack
    ///
    /// The return is in the form of a tuple `(Editor, CueStack)`.
    ///
    /// This is useful for when you need both mutable references, as it avoids
    /// double borrows for `Model`.
    ///
    /// # Examples
    ///
    /// ```
    /// use cues::model::full_model::Model;
    ///
    /// let mut model = Model::new();
    /// let (editor, cuestack) = model.full_mut();
    /// ```
    #[must_use]
    pub const fn full_mut(&mut self) -> (&mut Editor, &mut CueStack) {
        (&mut self.editor, &mut self.cuestack)
    }
}
