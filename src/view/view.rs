use crate::model::model::{CueStack};

use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, ListItem, List},
    Frame,
};

pub fn view(model: &mut CueStack, frame: &mut Frame) {
    // Vec of ListItems (which comes from a decompiled list of Cues)
    let items: Vec<ListItem> = model
        .cues // Cues of the model
        .iter() // Iterator across cues
        .map(|i| ListItem::new(i.name.as_str())) 
        .collect(); //maps to string name and transforms an iterator back to collection

    // The `List` widget is then built with those items.
    let list = List::new(items).highlight_symbol(">>");


    // Finally the widget is rendered using the associated state. `events.state` is
    // effectively the only thing that we will "remember" from this draw call.
    frame.render_stateful_widget(list, frame.area(), &mut model.current_cue);
}
