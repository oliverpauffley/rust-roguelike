use crate::prelude::*;

// TODO fix this function
pub fn cull_dead_particles(ecs: &mut World, ctx: &BTerm) {
    let mut dead_particles = Vec::new();
    let mut query = <(Entity, &mut ParticleLifetime)>::query();
    for (entity, mut particle) in query.iter_mut(ecs) {
        particle.lifetime_ms -= ctx.frame_time_ms;
        if particle.lifetime_ms < 0. {
            dead_particles.push(*entity);
        }
    }

    dead_particles.iter().for_each(|dead| {
        ecs.remove(*dead);
    });
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct ParticleRequest {
    x: i32,
    y: i32,
    fg: RGB,
    bg: RGB,
    glyph: FontCharType,
    lifetime: f32,
}

pub struct ParticleBuilder {
    requests: Vec<ParticleRequest>,
}

impl ParticleBuilder {
    pub fn new() -> ParticleBuilder {
        ParticleBuilder {
            requests: Vec::new(),
        }
    }

    pub fn request(
        &mut self,
        x: i32,
        y: i32,
        fg: RGB,
        bg: RGB,
        glyph: FontCharType,
        lifetime: f32,
    ) {
        self.requests.push(ParticleRequest {
            x,
            y,
            fg,
            bg,
            glyph,
            lifetime,
        });
    }
}

#[system]
#[write_component(Point)]
#[write_component(Render)]
#[write_component(ParticleLifetime)]
pub fn particle_spawn(
    commands: &mut CommandBuffer,
    #[resource] particle_builder: &mut ParticleBuilder,
) {
    //let = <(Entity, &Point, &Render, &ParticleLifetime)>::query();
    for new_particle in particle_builder.requests.iter() {
        let point = Point::new(new_particle.x, new_particle.y);
        let p = commands.push(());
        commands.add_component(p, point);
        commands.add_component(
            p,
            Render {
                color: ColorPair::new(new_particle.fg, new_particle.bg),
                glyph: new_particle.glyph,
                render_order: PARTICLE_RENDER_ORDER,
            },
        );
        commands.add_component(
            p,
            ParticleLifetime {
                lifetime_ms: new_particle.lifetime,
            },
        );
    }
    particle_builder.requests.clear();
}
