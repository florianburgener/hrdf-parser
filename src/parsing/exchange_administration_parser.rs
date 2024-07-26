// 1 file(s).
// File(s) read by the parser:
// UMSTEIGV
use std::error::Error;

use crate::{
    models::{ExchangeTimeAdministration, Model},
    parsing::{ColumnDefinition, ExpectedType, FileParser, ParsedValue, RowDefinition, RowParser},
    storage::ResourceStorage,
    utils::AutoIncrement,
};

pub fn parse(path: &str) -> Result<ResourceStorage<ExchangeTimeAdministration>, Box<dyn Error>> {
    log::info!("Parsing UMSTEIGV...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a AdministrationExchangeTime instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::OptionInteger32),
            ColumnDefinition::new(9, 14, ExpectedType::String),
            ColumnDefinition::new(16, 21, ExpectedType::String),
            ColumnDefinition::new(23, 24, ExpectedType::Integer16),
        ]),
    ]);
    let parser = FileParser::new(&format!("{path}/UMSTEIGV"), row_parser)?;

    let auto_increment = AutoIncrement::new();

    let data = parser
        .parse()
        .map(|x| x.map(|(_, _, values)| create_instance(values, &auto_increment)))
        .collect::<Result<Vec<_>, _>>()?;
    let data = ExchangeTimeAdministration::vec_to_map(data);

    Ok(ResourceStorage::new(data))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(
    mut values: Vec<ParsedValue>,
    auto_increment: &AutoIncrement,
) -> ExchangeTimeAdministration {
    let stop_id: Option<i32> = values.remove(0).into();
    let administration_1: String = values.remove(0).into();
    let administration_2: String = values.remove(0).into();
    let duration: i16 = values.remove(0).into();

    ExchangeTimeAdministration::new(
        auto_increment.next(),
        stop_id,
        administration_1,
        administration_2,
        duration,
    )
}
