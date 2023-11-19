// GLEIS, GLEIS_LV95, GLEIS_WGS
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{JourneyPlatform, Platform},
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowMatcher, RowParser},
};

pub fn load_journey_stop_platforms_and_platforms() -> Result<
    (
        Vec<Rc<JourneyPlatform>>,
        HashMap<(i32, i32), Vec<Rc<JourneyPlatform>>>,
        Vec<Rc<Platform>>,
        HashMap<(i32, i32), Rc<Platform>>,
    ),
    Box<dyn Error>,
> {
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        RowDefinition::new(ROW_A, RowMatcher::new(8, 9, "#", false), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 14, ExpectedType::Integer32),
            ColumnDefinition::new(16, 21, ExpectedType::String),
            // 23-30 with #
            ColumnDefinition::new(24, 30, ExpectedType::Integer32),
        ]),
        RowDefinition::new(ROW_B, RowMatcher::new(8, 9, "#", true), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            // 9-16 with #
            ColumnDefinition::new(10, 16, ExpectedType::Integer32),
            ColumnDefinition::new(18, -1, ExpectedType::String),
        ]),
    ]);
    let file_parser = FileParser::new("data/GLEIS", row_parser)?;

    let mut journey_platform = vec![];
    let mut platforms = vec![];
    let mut first_section_last_cursor_position = 0;

    for (id, bytes_read, mut values) in file_parser.parse() {
        match id {
            ROW_A => {
                first_section_last_cursor_position += bytes_read;

                let stop_id = i32::from(values.remove(0));
                let journey_id = i32::from(values.remove(0));

                journey_platform.push(Rc::new(JourneyPlatform::new(
                    stop_id,
                    journey_id,
                    String::from(values.remove(0)),
                    i32::from(values.remove(0)),
                )))
            }
            ROW_B => platforms.push(Rc::new(Platform::new(
                i32::from(values.remove(0)),
                i32::from(values.remove(0)),
                String::from(values.remove(0)),
            ))),
            _ => unreachable!(),
        }
    }

    println!("{}", first_section_last_cursor_position);
    let journey_platform_index = create_journey_platform_index(&journey_platform);
    let platforms_index = create_platforms_index(&platforms);

    Ok((
        journey_platform,
        journey_platform_index,
        platforms,
        platforms_index,
    ))
}

fn create_journey_platform_index(
    journey_platform: &Vec<Rc<JourneyPlatform>>,
) -> HashMap<(i32, i32), Vec<Rc<JourneyPlatform>>> {
    journey_platform
        .iter()
        .fold(HashMap::new(), |mut acc, item| {
            acc.entry((item.journey_id, item.stop_id))
                .or_insert(Vec::new())
                .push(Rc::clone(item));
            acc
        })
}

fn create_platforms_index(platforms: &Vec<Rc<Platform>>) -> HashMap<(i32, i32), Rc<Platform>> {
    platforms.iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert((item.stop_id, item.platform_index), Rc::clone(item));
        acc
    })
}
