use std::collections::HashMap;

#[derive(Default)]
pub struct RenderPipelineContainer {
    pipelines: HashMap<String, RenderPipeline>,
}

impl RenderPipelineContainer {
    pub fn insert(&mut self, name: &str, pipeline: RenderPipeline) {
        self.pipelines.insert(name.to_string(), pipeline);
    }

    pub fn get(&self, name: &str) -> Option<&RenderPipeline> {
        self.pipelines.get(name)
    }
}

pub struct RenderPipelineRunContext {}

trait Node {
    fn run(&self, context: &mut RenderPipelineRunContext);
}

#[derive(Default)]
pub struct RenderPipeline {
    nodes: Vec<Box<dyn Node>>,
}

impl RenderPipeline {
    pub fn run(&self, context: &mut RenderPipelineRunContext) {
        for node in self.nodes.iter() {
            node.run(context);
        }
    }
}
