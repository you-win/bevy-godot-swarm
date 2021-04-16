use bevy_ecs::prelude::*;
use bevy_ecs::schedule::RunOnce;
use gdnative::api::{Input, VisualServer};
use gdnative::prelude::*;

struct Renderable {
    rid: Rid,
    global_transform: Transform2D,
}

#[derive(Clone)]
struct RenderablesToRemove {
    vec: Vec<Rid>,
}

#[derive(Clone)]
struct InputQueue {
    vec: Vec<i32>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum Stages {
    Startup,
    Preupdate,
    Update,
    Postupdate,
}

#[derive(NativeClass, Default)]
#[inherit(Reference)]
#[user_data(user_data::MutexData<ECS>)]
pub struct ECS {
    schedule: Schedule,
    world: World,
}

#[methods]
impl ECS {
    fn new(_owner: &Reference) -> Self {
        let mut ecs = ECS {
            schedule: Schedule::default(),
            world: World::default(),
        };

        // Insert resources
        ecs.world
            .insert_resource(RenderablesToRemove { vec: vec![] });

        // Add stages
        ecs.schedule
            .add_stage(
                Stages::Startup,
                Schedule::default()
                    .with_run_criteria(RunOnce::default())
                    .with_stage(Stages::Startup, SystemStage::parallel()),
            )
            .add_stage(Stages::Preupdate, SystemStage::parallel())
            .add_stage(Stages::Update, SystemStage::parallel())
            .add_stage(Stages::Postupdate, SystemStage::parallel());

        // Add system
        ecs.schedule
            // Startup
            .stage(Stages::Startup, |schedule: &mut Schedule| {
                return schedule.add_system_to_stage(Stages::Startup, hello_world.system());
            })
            // Preupdate
            .add_system_to_stage(Stages::Postupdate, cleanup_rids.system())
            // Update
            .add_system_to_stage(Stages::Update, debug_move_right.system())
            // Postupdate
            .add_system_to_stage(Stages::Postupdate, cleanup_rids.system());
        // .add_system_to_stage(Stages::Postupdate, debug_print_positions.system());

        return ecs;
    }

    #[export]
    fn step(&mut self, _owner: &Reference, delta: f32) {
        // godot_print!("step {}", delta)
        self.schedule.run(&mut self.world);
    }

    #[export]
    fn register_entity(&mut self, _owner: &Reference, rid: Rid, global_transform: Transform2D) {
        self.world.spawn().insert(Renderable {
            rid: rid,
            global_transform: global_transform,
        });
    }

    #[export]
    fn unregister_entity_deferred(&mut self, _owner: &Reference, rid: Rid) {
        let mut renderables_to_remove = self
            .world
            .get_resource_mut::<RenderablesToRemove>()
            .unwrap();
        renderables_to_remove.vec.push(rid);
    }

    #[export]
    fn read_input(&mut self, _owner: &Reference) {
        let input_handler = Input::godot_singleton();
        if input_handler.is_action_pressed("move_up") {
            godot_print!("up!")
        }
        if input_handler.is_action_pressed("move_down") {
            godot_print!("down!")
        }
    }
}

/*
* Systems
*/

fn hello_world() {
    godot_print!("hello world");
}

fn cleanup_rids(
    mut commands: Commands,
    mut renderables_to_remove: ResMut<RenderablesToRemove>,
    query: Query<(Entity, &Renderable)>,
) {
    if renderables_to_remove.vec.len() == 0 {
        return;
    }
    for (entity, renderable) in query.iter() {
        if renderables_to_remove
            .vec
            .iter()
            .any(|&rid| rid == renderable.rid)
        {
            commands.entity(entity).despawn();
        }
    }

    renderables_to_remove.vec.clear();
}

#[warn(dead_code)]
fn debug_print_positions(query: Query<&Renderable>) {
    for r in query.iter() {
        godot_print!("{}, {}", r.global_transform.m31, r.global_transform.m32)
    }
}

#[warn(dead_code)]
fn debug_move_right(mut query: Query<&mut Renderable>) {
    for mut r in query.iter_mut() {
        // TODO can use translation() if we implement the associated type?
        r.global_transform.m31 += 1.0;
        unsafe {
            VisualServer::godot_singleton().canvas_item_set_transform(r.rid, r.global_transform);
        }
    }
}
