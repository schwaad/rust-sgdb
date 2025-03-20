pub enum ColumnType {
    Int,
    Str,
    Float,
    Char,
}

pub struct Column {
    pub column_type: ColumnType,
}

