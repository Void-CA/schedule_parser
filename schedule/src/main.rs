mod pipeline;
mod extraction;
mod domain;
mod analytics;
mod parser_qa;
mod validator;
mod error;
mod storage;

use std::collections::HashMap;
use domain::models::Class;

fn main() {
    let dataset_path = "json/full_schedule.json";
    
    // Cargar fuente de verdad (formato HashMap directo, retrocompatible)
    let dataset: HashMap<String, Vec<Class>> = 
        serde_json::from_str(&std::fs::read_to_string(dataset_path).expect("Error leyendo archivo"))
        .expect("Error parseando JSON");

    // Todas las clases planas
    let all_classes: Vec<Class> = dataset.values().flat_map(|c| c.clone()).collect();

    // Generar schedules derivados
    let professors_schedules = domain::professor::ProfessorSchedules::build(&all_classes);
    let rooms_schedules = domain::room::RoomSchedules::build(&all_classes);

    // Guardar perspectivas derivadas
    storage::json_store::save_json("professors_schedules.json", &professors_schedules.map)
        .expect("Error guardando professors_schedules");

    storage::json_store::save_json("rooms_schedules.json", &rooms_schedules.map)
        .expect("Error guardando rooms_schedules");

    println!("Generados: professors_schedules.json, rooms_schedules.json");
}