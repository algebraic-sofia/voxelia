use specs::{Component, DispatcherBuilder, World};
use specs::WorldExt;

/// This struct stores all the information needed to run a simulation of a voxelia world.
pub struct Engine<'a, 'b> {
    pub world: specs::World,
    pub dispatcher: specs::Dispatcher<'a, 'b>,
}

impl<'a, 'b> Engine<'a, 'b> {
    pub fn run(&mut self) {
        self.dispatcher.dispatch(&self.world)
    }
}

/// The [World] is a builder for both a specs::World and a specs::Dispatcher
pub struct WorldBuilder<'a, 'b> {
    dispatcher: specs::DispatcherBuilder<'a, 'b>,
    world: specs::World,
}

impl<'a, 'b> Default for WorldBuilder<'a, 'b> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, 'b> WorldBuilder<'a, 'b> {
    pub fn new() -> Self {
        WorldBuilder {
            dispatcher: DispatcherBuilder::new(),
            world: World::new(),
        }
    }

    /// Adds a new system to the world builder.
    pub fn with_system<T>(&mut self, system: T, name: &str, deps: &[&str])
    where
        T: for<'c> specs::System<'c> + Send + 'a,
    {
        self.dispatcher.add(system, name, deps);
    }

    /// Registers a new type of component into the world.
    pub fn with_component<T: Component>(&mut self)
    where
        T::Storage: Default,
    {
        self.world.register::<T>();
    }

    /// Adds a new resource into the system
    pub fn with_resource<R>(&mut self, resource: R)
    where
        R: specs::shred::Resource,
    {
        self.world.insert(resource);
    }
}

/// A plugin adds information to the ECS of the engine in order to add new systems and new things
/// to the game logic.
pub trait Plugin {
    fn setup(self, world: &mut WorldBuilder);
}

/// Creates a new Engine based on plugins
pub struct Builder<'a, 'b> {
    world_builder: WorldBuilder<'a, 'b>,
}

impl<'a, 'b> Default for Builder<'a, 'b> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, 'b> Builder<'a, 'b> {
    pub fn new() -> Self {
        Builder {
            world_builder: WorldBuilder::new(),
        }
    }

    /// Registers a new plugin into the engine
    pub fn with(mut self, plugin: impl Plugin + 'static) -> Self {
        plugin.setup(&mut self.world_builder);
        self
    }

    /// Creates a new engine using a bunch of plugins.
    pub fn build(self) -> Engine<'a, 'b> {
        let mut world = self.world_builder.world;
        let mut dispatcher = self.world_builder.dispatcher.build();
        dispatcher.setup(&mut world);

        Engine { world, dispatcher }
    }
}
