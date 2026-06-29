use rltbl_db::db_value::DbRow;
use rltbl_db_row::ConvertDbRow;
use rltbl_db_row_derive::ConvertDbRow;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, ConvertDbRow)]
struct MyStruct {
    _alpha: usize,
    _beta: String,
}

fn main() {
    let mystruct = MyStruct {
        _alpha: 1,
        _beta: "foo".to_string(),
    };
    let row = mystruct.into_db_row();
    let mystruct = MyStruct::from_db_row(row);
    println!("MYSTRUCT 1: {mystruct:?}");

    let row: DbRow = mystruct.into();
    let mystruct = MyStruct::from(row);
    println!("MYSTRUCT 2: {mystruct:?}");
}
