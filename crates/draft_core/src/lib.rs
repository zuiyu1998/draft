use fyrox_resource::manager::ResourceManager;

pub mod pool;

pub trait ImportResourcePlugin: Send + Sync + 'static {
    fn import(&self, resource_manager: &ResourceManager);
}
