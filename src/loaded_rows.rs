use std::ops::{Index, Range};

#[derive(Clone)]
pub enum RowState<T: Clone> {
    Placeholder,
    Loading,
    Loaded(T),
    Error(String),
}

impl<T: Clone> std::fmt::Debug for RowState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RowState::Placeholder => write!(f, "Placeholder"),
            RowState::Loading => write!(f, "Loading"),
            RowState::Loaded(_) => write!(f, "Loaded"),
            RowState::Error(e) => write!(f, "Error({})", e),
        }
    }
}

pub struct LoadedRows<T: Clone> {
    rows: Vec<RowState<T>>,
}

impl<T: Clone> LoadedRows<T> {
    pub fn new() -> Self {
        Self { rows: vec![] }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    #[inline]
    pub fn resize(&mut self, len: usize) {
        self.rows.resize(len, RowState::Placeholder);
    }

    pub fn write_loading(&mut self, range: Range<usize>) {
        if range.end > self.rows.len() {
            self.rows.resize(range.end, RowState::Placeholder);
        }

        for row in &mut self.rows[range] {
            *row = RowState::Loading;
        }
    }

    pub fn write_loaded(
        &mut self,
        loading_result: Result<(Vec<T>, Range<usize>), String>,
        missing_range: Range<usize>,
    ) {
        match loading_result {
            Ok((rows, range)) => {
                if range.end > self.rows.len() {
                    self.rows.resize(range.end, RowState::Placeholder);
                }

                for (self_row, loaded_row) in self.rows[range].iter_mut().zip(rows) {
                    *self_row = RowState::Loaded(loaded_row);
                }
            }
            Err(error) => {
                for row in &mut self.rows[missing_range] {
                    *row = RowState::Error(error.clone());
                }
            }
        }
    }

    #[inline]
    pub fn missing_range(&self, range: Range<usize>) -> Option<Range<usize>> {
        let do_load_predicate = |row| matches!(row, &RowState::Placeholder | &RowState::Error(_));

        let slice = &self.rows[range.clone()];

        let start = slice.iter().position(do_load_predicate)?;
        let end = slice.iter().rposition(do_load_predicate)?;

        let start = start + range.start;
        let end = end + range.start + 1;

        Some(start..end)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.rows.fill(RowState::Placeholder);
    }
}

impl<T: Clone> Index<Range<usize>> for LoadedRows<T> {
    type Output = [RowState<T>];

    #[inline]
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.rows[index]
    }
}

impl<T: Clone> Index<usize> for LoadedRows<T> {
    type Output = RowState<T>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}