use rltbl_db::{
    db_row,
    db_value::{DbRow, DbValue},
};
use rltbl_db_row::ConvertDbRow;
use serde::{Deserialize, Serialize};
use serde_json::json;

type Custom = String;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ConvertDbRow, PartialEq)]
struct MyStruct {
    alpha: usize,
    beta: Custom,
    #[json_value]
    gamma: serde_json::Value,
}

fn main() {
    let expected_mystruct = MyStruct {
        alpha: 1,
        beta: "foo".to_string(),
        gamma: json!(1),
    };
    let expected_db_row =
        db_row! { "alpha" => 1_i64, "beta" => "foo", "gamma" => DbValue::Json(json!(1)) };

    let row: DbRow = expected_mystruct.clone().into();
    assert_eq!(expected_db_row, row);
    let mystruct = MyStruct::from(row);
    assert_eq!(expected_mystruct, mystruct);
}
