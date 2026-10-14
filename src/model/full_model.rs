use crate::model::editor::Editor;
use crate::model::cuestack::CueStack;

pub struct Model {
    editor: Editor,
    cuestack: CueStack,
}

impl Model {
    pub const fn new(editor: Editor, cuestack: CueStack) -> Self {
        Self { editor, cuestack }
    }

    pub fn editor(&self) -> &Editor {
        &self.editor
    }

    pub fn set_editor(&mut self, editor: Editor) {
        self.editor = editor;
    }

    pub fn cuestack(&self) -> &CueStack {
        &self.cuestack
    }

    pub fn set_cuestack(&mut self, cuestack: CueStack) {
        self.cuestack = cuestack;
    }

    pub fn cuestack_mut(&mut self) -> &mut CueStack {
        &mut self.cuestack
    }

    pub fn editor_mut(&mut self) -> &mut Editor {
        &mut self.editor
    }

    pub fn full_mut(&mut self) -> (&mut Editor, &mut CueStack) {
        (&mut self.editor, &mut self.cuestack)
    }
}
