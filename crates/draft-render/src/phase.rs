use std::ops::{Deref, DerefMut};

use fxhash::FxHashMap;
use fyrox_core::ImmutableString;

use crate::{RenderWorld, frame_graph::RenderPassBuilder};

pub trait MeshRenderPhase: 'static {
    fn render_mesh(&mut self, render_pass_builder: &mut RenderPassBuilder, world: &RenderWorld);
}

impl<T: MeshRenderPhase> RenderPhase for T {
    fn name(&self) -> ImmutableString {
        ImmutableString::new("MeshRenderPhase")
    }

    fn render(&mut self, render_pass_builder: &mut RenderPassBuilder, world: &RenderWorld) {
        self.render_mesh(render_pass_builder, world);
    }
}

pub trait RenderPhase: 'static {
    fn name(&self) -> ImmutableString;

    fn render(&mut self, render_pass_builder: &mut RenderPassBuilder, world: &RenderWorld);
}

pub struct RenderPhaseContainer(Box<dyn RenderPhase>);

impl RenderPhaseContainer {
    pub fn new<T: RenderPhase>(phase: T) -> Self {
        Self(Box::new(phase))
    }

    pub fn name(&self) -> ImmutableString {
        self.0.name()
    }

    pub fn render(&mut self, render_pass_builder: &mut RenderPassBuilder, world: &RenderWorld) {
        self.0.render(render_pass_builder, world);
    }
}

#[derive(Default)]
pub struct PhasesContainer(FxHashMap<ImmutableString, RenderPhaseContainer>);

impl Deref for PhasesContainer {
    type Target = FxHashMap<ImmutableString, RenderPhaseContainer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PhasesContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
