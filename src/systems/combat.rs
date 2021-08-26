use crate::prelude::*;
use crate::systems::particle_system::ParticleBuilder;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(Carried)]
#[read_component(Point)]
pub fn combat(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] particle_system: &mut ParticleBuilder,
) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect();

    victims.iter().for_each(|(message, attacker, victim)| {
        let base_damage = if let Ok(v) = ecs.entry_ref(*attacker) {
            if let Ok(dmg) = v.get_component::<Damage>() {
                dmg.0
            } else {
                0
            }
        } else {
            0
        };

        let weapon_damage: i32 = <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, dmg)| dmg.0)
            .sum();

        let final_damage = base_damage + weapon_damage;

        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if let Ok(postion) = ecs.entry_ref(*victim).unwrap().get_component::<Point>() {
            particle_system.request(
                postion.x,
                postion.y,
                RGB::named(DARKRED),
                RGB::named(BLACK),
                to_cp437('█'),
                200.0,
            );
        }
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= final_damage;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
        }
        commands.remove(*message);
    });
}
