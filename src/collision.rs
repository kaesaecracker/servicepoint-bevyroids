use std::marker::PhantomData;

use bevy::{prelude::*, ecs::schedule::ScheduleLabel};

use crate::boundary::Bounding;

pub struct CollisionPlugin<Hittable, Hurtable> {
    _phantom: PhantomData<(Hittable, Hurtable)>,
}

impl<Hittable: Component, Hurtable: Component> CollisionPlugin<Hittable, Hurtable> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<Hittable: Component, Hurtable: Component> Plugin for CollisionPlugin<Hittable, Hurtable> {
    fn build(&self, app: &mut App) {
        app.add_event::<HitEvent<Hittable, Hurtable>>()
            .add_systems(Update, collision_system::<Hittable, Hurtable>.in_set(CollisionSystemLabel));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, ScheduleLabel)]
pub struct CollisionSystemLabel;

#[derive(Debug, Event)]
pub struct HitEvent<A, B> {
    entities: (Entity, Entity),
    _phantom: PhantomData<(A, B)>,
}

impl<A, B> HitEvent<A, B> {
    pub fn hittable(&self) -> Entity {
        self.entities.0
    }

    pub fn hurtable(&self) -> Entity {
        self.entities.1
    }
}

#[derive(Debug, Component)]
pub struct Collidable;

fn collision_system<A: Component, B: Component>(
    mut hits: EventWriter<HitEvent<A, B>>,
    hittables: Query<(Entity, &Transform, &Bounding), (With<Collidable>, With<A>)>,
    hurtables: Query<(Entity, &Transform, &Bounding), (With<Collidable>, With<B>)>,
) {
    for (hittable_entity, hit_transform, hit_bounds) in hittables.iter() {
        for (hurtable_entity, hurt_transform, hurt_bounds) in hurtables.iter() {
            let distance = (hit_transform.translation - hurt_transform.translation).length();
            if distance < **hit_bounds + **hurt_bounds {
                hits.send(HitEvent {
                    entities: (hittable_entity, hurtable_entity),
                    _phantom: PhantomData,
                });
            }
        }
    }
}
