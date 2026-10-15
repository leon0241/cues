use crate::model::editor::Editor;
use crate::model::cuestack::CueStack;

pub struct Model {
    editor: Editor,
    cuestack: CueStack,
}

impl Model {
    #[must_use]
    pub const fn new(editor: Editor, cuestack: CueStack) -> Self {
        Self { editor, cuestack }
    }

    #[must_use]
    pub const fn editor(&self) -> &Editor {
        &self.editor
    }

    pub fn set_editor(&mut self, editor: Editor) {
        self.editor = editor;
    }

    #[must_use]
    pub const fn cuestack(&self) -> &CueStack {
        &self.cuestack
    }

    pub fn set_cuestack(&mut self, cuestack: CueStack) {
        self.cuestack = cuestack;
    }

    #[must_use]
    pub const fn cuestack_mut(&mut self) -> &mut CueStack {
        &mut self.cuestack
    }

    #[must_use]
    pub const fn editor_mut(&mut self) -> &mut Editor {
        &mut self.editor
    }

    #[must_use]
    pub const fn full_mut(&mut self) -> (&mut Editor, &mut CueStack) {
        (&mut self.editor, &mut self.cuestack)
    }
}
