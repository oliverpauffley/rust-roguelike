// TODO: in a larger game you might want to write one system per effect along with a system to remove activation requests once all possible effects are processed.

use crate::prelude::*;

use super::particle_system::ParticleBuilder;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]
#[read_component(Point)]
pub fn use_items(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map,
    #[resource] particle_system: &mut ParticleBuilder,
) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();

    <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            let item = ecs.entry_ref(activate.item);
            if let Ok(item) = item {
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    healing_to_apply.push((activate.used_by, healing.amount));
                }

                if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {
                    map.revealed_tiles.iter_mut().for_each(|t| *t = true);
                }
            }
            commands.remove(activate.item); // remove the activate message
            commands.remove(*entity) // remove the item
        });

    for heal in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            // try and get the entity that used the item.
            if let Ok(health) = target.get_component_mut::<Health>() {
                // check if the entity has a health component.
                health.current = i32::min(health.max, health.current + heal.1);
            }

            if let Ok(position) = target.get_component::<Point>() {
                particle_system.request(
                    position.x,
                    position.y,
                    RGB::named(DARKSEAGREEN),
                    RGB::named(BLACK),
                    to_cp437('♥'),
                    200.0,
                );
            }
        }
    }
}
