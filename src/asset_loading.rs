#![allow(clippy::needless_pass_by_value)]
// https://github.com/maciekglowka/hike_deck/blob/main/src/assets.rs
use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::flow_control::GameState;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetList>().add_systems(
            Update,
            (check_asset_loading).run_if(in_state(GameState::LoadAssets)),
        );
    }
}

#[derive(Default, Resource)]
pub struct AssetList(pub Vec<UntypedHandle>);

fn check_asset_loading(
    asset_server: Res<AssetServer>,
    asset_list: Res<AssetList>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let status = asset_list
        .0
        .iter()
        .map(bevy::prelude::UntypedHandle::id)
        .fold(LoadState::Loaded, |general_status, asset_id| {
            let status = asset_server
                .get_load_state(asset_id)
                .expect("The asset do not exist");
            match status {
                LoadState::Failed(e) => LoadState::Failed(e),
                LoadState::Loaded if general_status != LoadState::Loaded => general_status,
                LoadState::Loaded => LoadState::Loaded,
                _ => LoadState::Loading,
            }
        });
    match status {
        LoadState::Loaded => {
            next_state.set(GameState::RunMainLoop);
        }
        LoadState::Failed(e) => {
            panic!("{e:?}");
        }
        _ => {}
    };
}
