use fyrox_core::{reflect::*, visitor::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub enum ShaderStage {
    #[default]
    Vertex,
    Fragment,
    Compute,
    Task,
    Mesh,
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub struct Shader {
    pub path: String,
    pub source: Source,
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize)]
pub enum Source {
    Wgsl(String),
    Glsl(String, ShaderStage),
}

impl Default for Source {
    fn default() -> Self {
        Self::Wgsl("".into())
    }
}

impl Shader {
    pub fn from_wgsl(source: impl Into<String>, path: impl Into<String>) -> Shader {
        let source = source.into();
        let path = path.into();
        Shader {
            path,
            source: Source::Wgsl(source),
        }
    }

    pub fn from_glsl(
        source: impl Into<String>,
        stage: ShaderStage,
        path: impl Into<String>,
    ) -> Shader {
        let source = source.into();
        let path = path.into();
        Shader {
            path,
            source: Source::Glsl(source, stage),
        }
    }
}
