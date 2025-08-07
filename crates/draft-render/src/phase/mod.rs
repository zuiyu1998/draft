mod mesh;

pub use mesh::*;

use fxhash::FxHashMap;
use fyrox_core::ImmutableString;

use crate::{MaterialRenderData, RenderWorld, frame_graph::RenderPassBuilder};

pub trait PhaseName {
    fn name() -> ImmutableString;
}

pub trait RenderPhase: 'static + PhaseName {
    fn get_material_render_data_mut(&mut self) -> &mut MaterialRenderData;

    fn render(&self, render_pass_builder: &mut RenderPassBuilder, world: &mut RenderWorld);
}

pub trait ErasedRenderPhase: 'static {
    fn name(&self) -> ImmutableString;

    fn render(&self, render_pass_builder: &mut RenderPassBuilder, world: &mut RenderWorld);
}

impl<T: RenderPhase> ErasedRenderPhase for T {
    fn name(&self) -> ImmutableString {
        <T as PhaseName>::name()
    }

    fn render(&self, render_pass_builder: &mut RenderPassBuilder, world: &mut RenderWorld) {
        <T as RenderPhase>::render(self, render_pass_builder, world);
    }
}

pub struct RenderPhases {
    name: ImmutableString,
    phases: Vec<Box<dyn ErasedRenderPhase>>,
}

impl RenderPhases {
    pub fn new(name: ImmutableString) -> Self {
        Self {
            name,
            phases: vec![],
        }
    }

    pub fn push<T: RenderPhase>(&mut self, value: T) {
        self.phases.push(Box::new(value));
    }

    pub fn name(&self) -> ImmutableString {
        self.name.clone()
    }

    pub fn render(&self, render_pass_builder: &mut RenderPassBuilder, world: &mut RenderWorld) {
        for phase in self.phases.iter() {
            phase.render(render_pass_builder, world);
        }
    }
}

#[derive(Default)]
pub struct RenderPhasesContainer(FxHashMap<ImmutableString, RenderPhases>);

impl RenderPhasesContainer {
    pub fn get_phases<T: RenderPhase>(&self) -> Option<&RenderPhases> {
        let key = T::name();
        self.get(&key)
    }

    pub fn get(&self, key: &ImmutableString) -> Option<&RenderPhases> {
        self.0.get(key)
    }

    pub fn push<T: RenderPhase>(&mut self, phase: T) {
        let name = T::name();

        self.0
            .entry(name.clone())
            .or_insert(RenderPhases::new(name))
            .push(phase);
    }
}
