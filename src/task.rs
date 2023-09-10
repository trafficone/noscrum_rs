use dynomite::{Item, Attributes};
use uuid::Uuid;
use serde::{Serialize,Deserialize};
use crate::noscrum_date::NoScrumDate;

#[derive(Attributes, Debug, Clone, Serialize, Deserialize)]
struct TaskWork {
    date: NoScrumDate,
    hours_worked: f32,
}

#[derive(Item, Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: Uuid,
    rectype: String,
    #[dynomite(sort_key)]
    composite_id: Vec<Uuid>,
    #[dynomite(partition_key)]
    user_id: Uuid,
    task_name: String,
    estimate: Option<f32>,
    sprint_id: Option<Uuid>,
    work: Vec<TaskWork>
}

impl Task {
    fn new(user_id: Uuid, epic_id: Uuid, story_id: Uuid, name: String, estimate: Option<f32>, sprint_id: Option<Uuid>) {
        let id = Uuid::new_v4();
        let task = Task{
            id,
            rectype: "task".to_owned(),
            user_id,
            composite_id: vec![epic_id, story_id, id],
            task_name: name,
            estimate,
            sprint_id,
            work: Vec::new(),
        };
        todo!("Create record in dynamodb");
    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_create_task(){
        todo!()
    }

    #[test]
    fn test_edit_task(){
        todo!()
    }

    #[test]
    fn test_add_work(){
        todo!()
    }
}