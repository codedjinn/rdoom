use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct EntityDefintion {
    name: String,
    animations: Vec<AnimationDefinition>
}

#[derive(Serialize, Deserialize)]
pub struct AnimationDefinition {
    name: String,
    frames: Vec<String>
}