mod pipeline;
mod extraction;
mod domain;
mod analytics;
mod parser_qa;
mod validator;
mod error;
mod storage;

use crate::pipeline::Pipeline;
use crate::analytics::model::ScheduleAnalytics;

use crate::storage::json_store::JsonStore;

fn main() {
    let dataset = Pipeline::build("horarios_IV.pdf");

    JsonStore::save("horarios_IV.json", &dataset)
        .expect("Error guardando JSON");

    let analytics = ScheduleAnalytics::new(dataset);

    analytics.summary();
}