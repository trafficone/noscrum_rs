use dynomite::{
    Attribute, Item
};
use uuid::Uuid;
use leptos::*;
use serde::{Serialize, Deserialize};
use crate::NoScrumError;

#[derive(Attribute, Debug, Clone, Serialize, Deserialize)]
enum EpicColor {
    Green,
    Blue,
    Orange,
    Purple,
    Red
}

impl EpicColor {
    fn get_color_name(&self) -> &str {
    // Returns a reference to the color name of this [`EpicColor`].
        match *self {
            EpicColor::Green => "green",
            EpicColor::Blue => "blue",
            EpicColor::Orange => "orange",
            EpicColor::Purple => "purple",
            EpicColor::Red => "red"
        }
    }
}

#[derive(Attribute, Debug, Clone, Serialize, Deserialize)]
enum EpicState {
    Active,
    Archived
}

 
#[derive(Item, Debug, Clone, Serialize, Deserialize)]
pub struct Epic {
    id: Uuid,
    rectype: String,
    #[dynomite(partition_key)]
    user_id: Uuid,
    #[dynomite(sort_key)]
    epic_name: String,
    epic_color: EpicColor,
    epic_state: EpicState
}

impl Epic {
    fn new(user_id: Uuid, name: String, color: EpicColor) -> Result<Epic, NoScrumError> {
        let new_epic = Epic {
            id: Uuid::new_v4(),
            rectype: "epic".to_owned(),
            user_id,
            epic_name: name,
            epic_color: color,
            epic_state: EpicState::Active
        };
        todo!("Attempt to create new epic in dynamodb, but fail on duplicate epic_name");
    }

    fn archive(&self) {
        todo!("set epic_state to Archive");
    }
}

#[server(CreateEpic,"/api")]
async fn create_epic(cx: Scope, name: String, color: String) -> Result<Epic, ServerFnError> {
    let new_epic = Epic::new(cx.user.id, name, color);
    match new_epic {
        Epic => Ok(new_epic),
        NoScrumError(error) => {Err(error)}
    }
} 

#[server(UpdateEpic,"/api")]
async fn update_epic(cx: Scope, id: Uuid, name: Option<String>, color: Option<String>, archive: Option<bool>) -> Result<Epic, ServerFnError> {
    todo!("update epic name and/or color");
}

#[component]
fn epic_container(cx: Scope) -> impl IntoView{
    todo!("Container for epic");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_epic (){
        todo!()
    }

    #[test]
    fn test_update_epic(){
        todo!()
    }

    #[test]
    fn test_archive_epic(){
        todo!()
    }
}