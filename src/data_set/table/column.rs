pub enum ColumnType {
    Int,
    Str,
    Float,
    Char,
}

pub struct Column {
    pub column_name : String,
    pub column_type : ColumnType,
}

