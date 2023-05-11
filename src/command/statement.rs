pub enum PrepareStatus {
    Success,
    UnrecognizedStatement,
}
// enum StatementType {
//     Select,
//     Insert,
//     Update,
//     Delete,
// }

// pub struct PreparedStatement {
//     statement_type: StatementType,
//     statement: String,
// }

// fn get_statement_type(statement: &str) -> Result<StatementType, ()> {
//     let args = statement.trim().to_lowercase();
//     let tokens: Vec<String> = vec![args.split(" ").collect()];

//     match tokens[0].as_str() {
//         "select" => Ok(StatementType::Select),
//         "insert" => Ok(StatementType::Insert),
//         "update" => Ok(StatementType::Update),
//         "delete" => Ok(StatementType::Delete),
//         _ => Err(()),
//     }
// }

pub fn do_prepared_statement(statement: &str) -> PrepareStatus {
    println!("{}", statement);
    PrepareStatus::Success
}
