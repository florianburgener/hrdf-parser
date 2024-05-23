use std::{cell::RefCell, collections::HashMap, rc::Rc};

use chrono::{Days, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::{
    hrdf::Hrdf,
    models::{Journey, Model, ResourceCollection, ResourceIndex, TimetableMetadataEntry},
};

// ------------------------------------------------------------------------------------------------
// --- SimpleResourceStorage
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleResourceStorage<M: Model<M>> {
    rows: ResourceCollection<M>,
    primary_index: ResourceIndex<M, M::K>,
}

#[allow(unused)]
impl<M: Model<M>> SimpleResourceStorage<M> {
    pub fn new(rows: ResourceCollection<M>) -> Self {
        let primary_index = M::create_primary_index(&rows);

        Self {
            rows,
            primary_index,
        }
    }

    pub fn rows(&self) -> &ResourceCollection<M> {
        &self.rows
    }

    pub fn primary_index(&self) -> &ResourceIndex<M, M::K> {
        &self.primary_index
    }

    pub fn find_by_id(&self, k: M::K) -> Rc<M> {
        Rc::clone(self.primary_index().get(&k).unwrap())
    }
}

// ------------------------------------------------------------------------------------------------
// --- TimetableMetadataStorage
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct TimetableMetadataStorage {
    rows: ResourceCollection<TimetableMetadataEntry>,
    primary_index: ResourceIndex<TimetableMetadataEntry>,
    find_by_key_index: RefCell<ResourceIndex<TimetableMetadataEntry, String>>,
}

#[allow(unused)]
impl TimetableMetadataStorage {
    pub fn new(rows: ResourceCollection<TimetableMetadataEntry>) -> Self {
        let primary_index = TimetableMetadataEntry::create_primary_index(&rows);

        Self {
            rows,
            primary_index,
            find_by_key_index: RefCell::new(HashMap::new()),
        }
    }

    pub fn rows(&self) -> &ResourceCollection<TimetableMetadataEntry> {
        &self.rows
    }

    pub fn primary_index(&self) -> &ResourceIndex<TimetableMetadataEntry> {
        &self.primary_index
    }

    pub fn find_by_key(&self, k: &str) -> Rc<TimetableMetadataEntry> {
        Rc::clone(self.find_by_key_index.borrow().get(k).unwrap())
    }

    pub fn set_find_by_key_index(&self, _: &Hrdf) {
        *self.find_by_key_index.borrow_mut() =
            self.rows().iter().fold(HashMap::new(), |mut acc, item| {
                acc.insert(item.key().to_owned(), Rc::clone(&item));
                acc
            });
    }

    pub fn start_date(&self) -> Rc<TimetableMetadataEntry> {
        self.find_by_key("start_date")
    }

    pub fn end_date(&self) -> Rc<TimetableMetadataEntry> {
        self.find_by_key("end_date")
    }

    pub fn num_days_between_start_and_end_date(&self) -> usize {
        let x = (self.end_date().value_as_NaiveDate() - self.start_date().value_as_NaiveDate())
            .num_days();
        usize::try_from(x).unwrap() + 1
    }
}

// ------------------------------------------------------------------------------------------------
// --- JourneyStorage
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct JourneyStorage {
    rows: ResourceCollection<Journey>,
    primary_index: ResourceIndex<Journey>,
    operating_journeys_index: RefCell<HashMap<NaiveDate, Vec<Rc<Journey>>>>,
}

#[allow(unused)]
impl JourneyStorage {
    pub fn new(rows: ResourceCollection<Journey>) -> Self {
        let primary_index = Journey::create_primary_index(&rows);

        Self {
            rows,
            primary_index,
            operating_journeys_index: RefCell::new(HashMap::new()),
        }
    }

    pub fn rows(&self) -> &ResourceCollection<Journey> {
        &self.rows
    }

    pub fn primary_index(&self) -> &ResourceIndex<Journey> {
        &self.primary_index
    }

    pub fn set_operating_journeys_index(&self, hrdf: &Hrdf) {
        let start_date = hrdf.timetable_metadata().start_date().value_as_NaiveDate();
        let num_days = hrdf
            .timetable_metadata()
            .num_days_between_start_and_end_date();

        let dates: Vec<NaiveDate> = (0..num_days)
            .into_iter()
            .map(|i| {
                start_date
                    .checked_add_days(Days::new(i.try_into().unwrap()))
                    .unwrap()
            })
            .collect();

        // *self.operating_journeys_index.borrow_mut() =
        let x = self.rows().iter().fold(HashMap::new(), |mut acc, item| {
            let bit_field = item.bit_field();
            let indexes: Vec<usize> = if let Some(bit_field) = bit_field {
                bit_field
                    .bits()
                    .iter()
                    .enumerate()
                    .filter(|(i, &x)| *i < num_days && x == 1)
                    .map(|(i, _)| i)
                    .collect()
            } else {
                (0..num_days).collect()
            };

            indexes.into_iter().for_each(|i| {
                acc.entry(dates[i].to_owned())
                    .or_insert(Vec::new())
                    .push(Rc::clone(&item));
                let x = acc.get(&dates[i]).unwrap();
                // println!("{} {} {} {}", acc.len(), mem::size_of_val(&acc), &acc.get(&dates[i]).unwrap().len(), mem::size_of_val(acc.get(&dates[i]).unwrap()));
            });

            acc
        });
        println!("Done {}", x.len());
        *self.operating_journeys_index.borrow_mut() = x;
    }
}
