mod pipeline;
mod extraction;
mod domain;
mod analytics;
mod parser_qa;
mod validator;
mod error;


use crate::pipeline::Pipeline;
use crate::analytics::model::ScheduleAnalytics;

fn main() {
    let dataset = Pipeline::build("horarios_IV.pdf");

    let analytics = ScheduleAnalytics::new(dataset);

    for (day, count) in analytics.classes_per_day() {
        println!("{}: {} clases", day, count);
    }
}