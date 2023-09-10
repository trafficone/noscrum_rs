use dynomite::{Item, Attributes};
use uuid::Uuid;
use serde::{Serialize,Deserialize};
use crate::noscrum_date::NoScrumDate;
use crate::sprint_schedule::SprintSchedule;

#[derive(Item, Debug, Clone, Serialize, Deserialize)]
pub struct Sprint {
    id: Uuid,
    rectype: String,
    #[dynomite(partition_key)]
    user_id: Uuid,
    start_date: NoScrumDate,
    end_date: NoScrumDate,
    schedule: Vec<SprintSchedule>,
}

impl Sprint {
    fn new(user_id: Uuid, start_date: NoScrumDate, end_date: NoScrumDate){
        let sprint = Sprint{
            id: Uuid::new_v4(),
            rectype: "sprint".to_owned(),
            user_id,
            start_date,
            end_date,
            schedule: Vec::new(),
        };
        todo!("Create record in Dynamodb");
    }
}