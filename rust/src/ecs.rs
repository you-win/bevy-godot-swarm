use std::collections::{vec_deque::Drain, VecDeque};

use bevy_ecs::prelude::*;
use bevy_ecs::schedule::RunOnce;
use gdnative::api::{Input, VisualServer};
use gdnative::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
struct Cleanup;

struct Renderable {
    rid: Rid,
    global_transform: Transform2D,
}

#[derive(Clone)]
struct RenderablesToRemove {
    vec: Vec<Rid>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum GodotInput {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

#[derive(Clone)]
struct InputQueue {
    queue: VecDeque<GodotInput>,
}

impl InputQueue {
    pub fn new() -> Self {
        let mut queue: VecDeque<GodotInput> = VecDeque::new();
        queue.make_contiguous();
        return InputQueue { queue: queue };
    }

    pub fn add(&mut self, data: GodotInput) {
        self.queue.push_back(data);
    }

    #[warn(dead_code)]
    pub fn read_single(&mut self) -> Option<GodotInput> {
        return self.queue.pop_front();
    }

    pub fn read_all(&mut self) -> Drain<'_, GodotInput> {
        return self.queue.drain(..);
    }
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
        ecs.world.insert_resource(InputQueue::new());

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
            .add_system_to_stage(Stages::Preupdate, cleanup_rids.system().label(Cleanup))
            .add_system_to_stage(Stages::Preupdate, handle_input.system().after(Cleanup))
            // Update
            .add_system_to_stage(Stages::Update, move_player.system());
        // Postupdate
        // .add_system_to_stage(Stages::Postupdate, debug_print_positions.system());

        return ecs;
    }

    #[export]
    fn step(&mut self, _owner: &Reference, _delta: f32) {
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
        let mut input_queue = self.world.get_resource_mut::<InputQueue>().unwrap();
        let input_handler = Input::godot_singleton();
        if input_handler.is_action_pressed("move_up") {
            input_queue.add(GodotInput::MoveUp);
        }
        if input_handler.is_action_pressed("move_down") {
            input_queue.add(GodotInput::MoveDown);
        }
        if input_handler.is_action_pressed("move_left") {
            input_queue.add(GodotInput::MoveLeft);
        }
        if input_handler.is_action_pressed("move_right") {
            input_queue.add(GodotInput::MoveRight);
        }
    }
}

/*
* Systems
*/

// TODO debug
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
            unsafe {
                VisualServer::godot_singleton().free_rid(renderable.rid);
            }
        }
    }

    renderables_to_remove.vec.clear();
}

fn handle_input(mut input_queue: ResMut<InputQueue>, mut query: Query<&mut Renderable>) {
    for mut r in query.iter_mut() {
        for input in input_queue.read_all() {
            match input {
                GodotInput::MoveUp => r.global_transform.m32 -= 1.0,
                GodotInput::MoveDown => r.global_transform.m32 += 1.0,
                GodotInput::MoveLeft => r.global_transform.m31 -= 1.0,
                GodotInput::MoveRight => r.global_transform.m31 += 1.0,
            }
        }
    }
}

#[warn(dead_code)]
fn debug_print_positions(query: Query<&Renderable>) {
    for r in query.iter() {
        godot_print!("{}, {}", r.global_transform.m31, r.global_transform.m32)
    }
}

#[warn(dead_code)]
fn move_player(query: Query<&Renderable>) {
    for r in query.iter() {
        unsafe {
            VisualServer::godot_singleton().canvas_item_set_transform(r.rid, r.global_transform);
        }
    }
}
