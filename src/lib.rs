#![doc = include_str!("../README.md")]
mod error;
mod hrdf;
mod models;
mod parsing;
mod storage;
mod utils;

pub use error::HrdfError as Error;
pub use hrdf::AddableTypes;
pub use hrdf::Hrdf;
pub use hrdf::ModifiableTypes;
pub use hrdf::RemovableTypes;
pub use models::*;
pub use storage::DataStorage;
pub use utils::timetable_end_date;
pub use utils::timetable_start_date;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RemovableTypes;
    use chrono::NaiveDate;
    use std::collections::HashMap;
    use test_log::test;

    #[test(tokio::test)]
    async fn url_not_found() {
        let hrdf = Hrdf::new(
            Version::V_5_40_41_2_0_6,
            "https://data.opentransportdata.swiss/test-should-not-exists",
            true,
            None,
        )
        .await;
        match hrdf {
            Ok(_) => panic!("should be an error"),
            Err(err) => {
                assert!(
                    err.to_string().to_lowercase().contains("404 not found"),
                    "The error should indicate '404 Not Found'"
                );
            }
        }
    }

    // #[test(tokio::test)]
    // #[ignore]
    // async fn parsing_2020() {
    //     let _hrdf = Hrdf::try_from_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(), true, None)
    //         .await
    //         .unwrap();
    // }
    //
    // #[test(tokio::test)]
    // #[ignore]
    // async fn parsing_2021() {
    //     let _hrdf = Hrdf::try_from_date(NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(), true, None)
    //         .await
    //         .unwrap();
    // }

    #[test(tokio::test)]
    #[ignore]
    async fn parsing_2022() {
        let _hrdf = Hrdf::try_from_date(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(), true, None)
            .await
            .unwrap();
    }

    #[test(tokio::test)]
    #[ignore]
    async fn parsing_2023() {
        let _hrdf = Hrdf::try_from_date(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(), true, None)
            .await
            .unwrap();
    }

    #[test(tokio::test)]
    #[ignore]
    async fn parsing_2024() {
        let _hrdf = Hrdf::try_from_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), true, None)
            .await
            .unwrap();
    }

    #[test(tokio::test)]
    #[ignore]
    async fn parsing_2025() {
        let _hrdf = Hrdf::try_from_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(), true, None)
            .await
            .unwrap();
    }

    #[test(tokio::test)]
    #[ignore]
    async fn parsing_2026() {
        let _hrdf = Hrdf::try_from_date(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(), true, None)
            .await
            .unwrap();
    }

    #[test(tokio::test)]
    #[ignore]
    async fn parsing_from_year_2026() {
        let _hrdf = Hrdf::try_from_year(2026, false, None).await.unwrap();
    }

    #[test(tokio::test)]
    async fn filtering_lines_and_stops_2025() {
        let filter = HashMap::from([
            (RemovableTypes::Line, vec!["41", "12", "9", "5", "14", "80"]),
            (
                RemovableTypes::Stop,
                vec!["Genève, Jonction", "Genève, Rive", "Genève, Bel-Air"],
            ),
        ]);
        let _hrdf = Hrdf::try_from_date(NaiveDate::from_ymd_opt(2026, 1, 24).unwrap(), true, None)
            .await
            .unwrap();
        let before_lines = _hrdf.data_storage().lines().data().len();
        let before_journeys = _hrdf.data_storage().journeys().data().len();
        let before_stops = _hrdf.data_storage().stops().data().len();
        let before_total_journey_nb = _hrdf
            .data_storage()
            .journeys_by_stop_id_and_bit_field_id()
            .iter()
            .fold(0, |acc, (_k, v)| acc + v.len());

        let filtered_hrdf = _hrdf.filter(&filter).unwrap();

        let after_lines = filtered_hrdf.data_storage().lines().data().len();
        let after_journeys = filtered_hrdf.data_storage().journeys().data().len();
        let after_stops = filtered_hrdf.data_storage().stops().data().len();
        let after_total_journey_nb = filtered_hrdf
            .data_storage()
            .journeys_by_stop_id_and_bit_field_id()
            .iter()
            .fold(0, |acc, (_k, v)| acc + v.len());
        assert!(before_lines >= after_lines);
        assert!(before_journeys >= after_journeys + 6);
        assert!(before_stops >= after_stops + 3);
        // Checks that something has been deleted
        assert!(before_total_journey_nb > after_total_journey_nb + 6);
        println!("removed {} journeys", before_total_journey_nb - after_total_journey_nb);

        // Test that it doesn't crash even when no journey exists at a stop
        let stop_id = 8592874; // This id should correspond to Palladium
        let data_storage = filtered_hrdf.data_storage();
        let date = NaiveDate::from_ymd_opt(2025, 4, 17).unwrap();
        let default_journey = Journey::default();
        let mut left_unfound_journeys = 0;

        let _stop_bit_field = data_storage.bit_fields_by_stop_id().get(&stop_id).unwrap();
        let bit_fields_2 = data_storage.bit_fields_by_day().get(&date).unwrap();

        let found_journeys =
            data_storage
                .bit_fields_by_stop_id()
                .get(&stop_id)
                .map_or(Vec::new(), |bit_fields_1| {
                    let bit_fields: Vec<_> = bit_fields_1.intersection(bit_fields_2).collect();

                    bit_fields
                        .into_iter()
                        .flat_map(|&bit_field_id| {
                            data_storage
                                .journeys_by_stop_id_and_bit_field_id()
                                .get(&(stop_id, bit_field_id))
                                .unwrap()
                        })
                        .map(|&journey_id| {
                            data_storage.journeys().find(journey_id).unwrap_or_else(|| {
                                eprintln!("Journey {:?} not found.", journey_id);
                                left_unfound_journeys += 1;
                                &default_journey
                            })
                        })
                        .collect()
                });
        assert_eq!(found_journeys.len(), 0);
        assert_eq!(left_unfound_journeys, 0);
    }
}
