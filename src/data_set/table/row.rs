use std::collections::BTreeMap;

pub struct Row {
    pub index: i32,
    pub cells: BTreeMap<i32,String>,
}

