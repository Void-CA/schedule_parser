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
    let dataset_I = Pipeline::build("horarios_I.pdf");
    let dataset_II = Pipeline::build("horarios_II.pdf");
    let dataset_III = Pipeline::build("horarios_III.pdf");
    let dataset_IV = Pipeline::build("horarios_IV.pdf");
    
    let mut students_schedule = domain::students::StudentSchedule::new();

    students_schedule.add_year(1, dataset_I.clone());
    students_schedule.add_year(2, dataset_II.clone());
    students_schedule.add_year(3, dataset_III.clone());
    students_schedule.add_year(4, dataset_IV.clone());
    
    // Save individual year JSONs
    JsonStore::save("json/estudiantes/horarios_I.json", &dataset_I).expect("Error");
    JsonStore::save("json/estudiantes/horarios_II.json", &dataset_II).expect("Error");
    JsonStore::save("json/estudiantes/horarios_III.json", &dataset_III).expect("Error");
    JsonStore::save("json/estudiantes/horarios_IV.json", &dataset_IV).expect("Error");
    
    let mut dataset = dataset_I;
    dataset.extend(dataset_II);
    dataset.extend(dataset_III);
    dataset.extend(dataset_IV);
    
    let professors_schedules = domain::professor::ProfessorSchedules::build(&dataset);
    JsonStore::save("professors_schedules.json", &professors_schedules.map)
        .expect("Error guardando JSON");

    JsonStore::save("json/estudiantes/horarios_full.json", &students_schedule.by_year).expect("Error")
}