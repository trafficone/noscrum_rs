use dynomite::{Item, Attribute};
use uuid::Uuid;
use serde::{Serialize,Deserialize};
use crate::noscrum_date::NoScrumDate;
use crate::NoScrumError;

#[derive(Attribute, Debug, Clone, Serialize, Deserialize)]
enum StoryStatus {
    Active,
    Archived
}


#[derive(Item, Debug, Clone, Serialize, Deserialize)]
struct Story{
    id: Uuid,
    rectype: String,
    #[dynomite(sort_key)] 
    composite_id: Vec<Uuid>,
    story_name: String,
    #[dynomite(partition_key)]
    user_id: Uuid,
    estimate: Option<f32>,
    deadline: Option<NoScrumDate>,
    status: StoryStatus
}

impl Story {
    fn new(user_id: Uuid, epic: Uuid, name: String, estimate: Option<f32>, deadline: Option<NoScrumDate>) -> Result<Story, NoScrumError> {
        let id = Uuid::new_v4();
        let new_story = Story{
            id,
            rectype: "story".to_owned(),
            composite_id: vec![epic, id],
            story_name: name,
            user_id,
            estimate,
            deadline,
            status: StoryStatus::Active
        };
        todo!("Try to create record in dynamodb");
    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_create_story(){
        todo!()
    }

    #[test]
    fn test_update_story(){
        todo!()
    }

    #[test]
    fn test_archive_story(){
        todo!()
    }

    #[test]
    fn test_get_stories_in_epic(){
        todo!()
    }
}