use dynomite::{Item, Attributes};
use uuid::Uuid;
use serde::{Serialize,Deserialize};
use crate::noscrum_date::NoScrumDate;
use crate::sprint_schedule::SprintSchedule;

#[derive(Attributes, Debug, Clone, Serialize, Deserialize)]
pub struct SprintSchedule {
    task_id: Uuid,
    date: NoScrumDate,
    order: u8,
}

impl SprintSchedule{
    todo!();
}