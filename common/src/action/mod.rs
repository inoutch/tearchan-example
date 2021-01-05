/*

fn move_to_destination(
    // entity
    entity_id: &EntityId,
    // action creator state
    start: &Position,
    end: &Position,
    // components
    character_status: &ComponentGroup<CharacterStatus>,
    map_path: &MapPath,
    ground: &Ground,
) -> ActionCreatorResult<CustomActionCreatorCommonState> {
    let target_character_status = character_status.get(entity_id).unwrap();
    let action_states = match map_path.search_paths(start, end) {
        Some(paths) => paths,
        None => return ActionCreatorResult::Cancel,
    }.map(path => {
        let movement_time = calc_movement_time(target_character_status, ground, &path.start);
        ActionState::new(*entity_id, movement_time, CustomActionState::MoveToPointActionState::new(*path))
    });

    if actions.is_empty() {
        return ActionCreatorResult::Complete;
    }

    ActionCreatorResult::Progressing { action_states }
}

fn bring_item_to_destination(
    entity_id: &EntityId,
    food_entity_id: &EntityId,
    positions: &ComponentGroup<Position>,
) -> ActionCreatorResult<CustomActionCreatorCommonState> {
    let action_creators = vec![
        CustomActionCreatorState::MoveToDestination { }
    ];
}

*/
