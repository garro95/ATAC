use crate::app::app::App;
use crate::app::ui::request::request_to_tree_item;
use crate::request::collection::Collection;
use ratatui::layout::Rect;
use ratatui::prelude::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;
use tui_tree_widget::{Tree, TreeItem};


pub fn collection_to_tree_item<'a>(collection: &Collection, identifier: usize) -> TreeItem<'a, usize> {
    let name = collection.name.clone();

    let line = Line::from(vec![
        Span::raw(name),
        Span::from(format!(" ({})", collection.requests.len())),
    ]);

    let items: Vec<TreeItem<usize>> = collection
        .requests
        .iter()
        .enumerate()
        .map(|(request_index, request)| request_to_tree_item(&request.read(), request_index))
        .collect();

    TreeItem::new(identifier, line, items).unwrap()
}

impl<'a> App<'a> {
    pub(super) fn render_collections(&mut self, frame: &mut Frame, rect: Rect) {
        let items: Vec<TreeItem<'a, usize>> = self
            .collections
            .iter()
            .enumerate()
            .map(|(collection_index, request)| collection_to_tree_item(request, collection_index))
            .collect();

        let tree_items = self.collections_tree.items.clone();

        let collections_tree = Tree::new(&tree_items)
            .unwrap()
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .node_closed_symbol("")
            .node_no_children_symbol("")
            .block(Block::default().title("Collections").borders(Borders::ALL));

        self.collections_tree.items = items;

        frame.render_stateful_widget(collections_tree, rect, &mut self.collections_tree.state);
    }
}
