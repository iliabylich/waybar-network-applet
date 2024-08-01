use gtk4::prelude::BoxExt;

use crate::widgets::Row;

pub(crate) struct RowList {
    rows: Vec<Row>,
}

impl RowList {
    pub(crate) fn new() -> Self {
        Self { rows: vec![] }
    }

    pub(crate) fn row(mut self, row: Row) -> Self {
        self.rows.push(row);
        self
    }

    pub(crate) fn rows(mut self, rows: impl Iterator<Item = Row>) -> Self {
        for row in rows {
            self.rows.push(row);
        }
        self
    }

    pub(crate) fn build(self) -> gtk4::Box {
        let list = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .css_classes(["row-list"])
            .build();

        for (idx, row) in self.rows.into_iter().enumerate() {
            list.append(&row.idx(idx).build());
        }

        list
    }
}
