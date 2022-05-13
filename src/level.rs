use bevy::prelude::*;
//use heron::prelude::*; 

#[derive(Component)]
struct Level {
    pub name: String,
    pub isLoaded: bool,
    pub width: u32,
    pub height: u32,
    tiles: Vec<Tile>,   
}

enum TileType {
    None,
    Floor,
    Wall
}

struct Tile {
    pub x: u32,
    pub y: u32,
    pub tile_type: TileType
}

impl Level {
    pub fn new(name:&str, width: u32, height: u32) -> Self {
        let mut tiles = Vec::new();

        let mut result = Level {
            name: String::from(name),
            isLoaded: false,
            width, height,
            tiles,
        };
        result.init();

        return result;
    }

    pub fn get(&self, x: u32, y: u32) -> &Tile {
        if x > self.width - 1 || y > self.height - 1 {
            panic!("x or y was out of bounds");
        }
        return &self.tiles[self.get_index(x,y)];
    }

    pub fn set(&mut self, x: u32, y: u32, tile_type: TileType) {
        let tile = Tile {
          x,y,tile_type  
        };
        let index = self.get_index(x,y);
        self.tiles[index] = tile;
    }

    fn init(&mut self) {
        for i in 0..self.level_size() {
            let coord = self.get_from(i);
            self.tiles.push(Tile {
                x: coord.0,
                y: coord.1,
                tile_type: TileType::Floor
            });
        }
    }

    fn get_from(&self, index: usize) -> (u32, u32) {
        let u32_index = index as u32;
        (u32_index / self.width, u32_index % self.width)
    }

    fn level_size(&self) -> usize {
        (self.width * self.height) as usize
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (self.width * y + x) as usize
    }
}

pub struct LevelPlugin;


impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(demo_level);
        //     .add_system(player_movement);
    }
}

fn demo_level(mut commands: Commands) {

    let mut level = Level::new("demo", 128, 128);

    commands.spawn()
            .insert(level);

}

fn render_level(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut query_level: Query<&Level>,
) {
    let level = query_level.single();


    // if (level.isLoaded) {

    //     let tile_handle = assets.load("textures/tiles/floor4_6.png");

    //     let mut sprites = vec![];
    //     sprites.push(SpriteBundle {
    //                         texture: tile_handle.clone(),
    //                         transform: Transform {
    //                             translation: Vec3::new(0.0,0.0,-0.05),
    //                             rotation: Quat::IDENTITY,
    //                             scale: Vec3::ONE,
    //                         },
    //                         sprite: Sprite {
    //                             custom_size: Some(Vec2::new(32.0, 32.0)),
    //                             ..default()
    //                         },
    //                         ..default()
    //                     });

    //     commands.spawn_batch(sprites);
    // }
}
// fn load_level(
//     mut commands: Commands,
//     assets: Res<AssetServer>
// ) {
//     let transform = Transform {
//         translation: Vec3::new(0.0, 0.0, 0.0),
//         rotation: Quat::IDENTITY,
//         scale: Vec3::ONE,
//     };
    
//     let sprite = SpriteBundle {
//         transform,
//         texture: assets.load("textures/dude/playb5.png"),
//         ..Default::default()
//     };

//     commands.spawn()
//         .insert_bundle(sprite);

//     let tile_size = 32.0;

    
//     let tex = assets.load("textures/tiles/floor4_6.png");

//     let mut sprites = vec![];
//     for y in 0..10 {
//         for x in 0..10 {

//             let pos = Vec2::new(x as f32, y as f32);
//             let translation = Vec3::new(100.0, 100.0, -0.05); // Vec3::new(pos.x * tile_size, pos.y * tile_size, 0.0);
//             let scale = Vec3::ONE;
//             let rotation = Quat::IDENTITY;

//             sprites.push(SpriteBundle {
//                 texture: tex.clone(),
//                 transform: Transform {
//                     translation,
//                     rotation,
//                     scale,
//                 },
//                 sprite: Sprite {
//                     custom_size: Some(Vec2::new(tile_size, tile_size)),
//                     ..default()
//                 },
//                 ..default()
//             });
//         }
//     }

//     commands.spawn_batch(sprites);
// }


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    #[should_panic(expected = "x or y was out of bounds")]
    fn check_level_get_out_of_bounds_1() {
        let mut new_level = Level::new("", 10, 10);
        new_level.get(5,15);
    }

    #[test]
    #[should_panic(expected = "x or y was out of bounds")]
    fn check_level_get_out_of_bounds_2() {
        let mut new_level = Level::new("", 10, 10);
        new_level.get(15,5);
    }

    #[test]
    fn should_get_tile() {
        let mut new_level = Level::new("", 10, 10);
        let tile = new_level.get(5, 5);
        assert_eq!(tile.x, 5);
        assert_eq!(tile.y, 5);
    }

}