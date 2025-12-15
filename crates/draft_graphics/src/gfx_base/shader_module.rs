use wgpu::ShaderModule as WgpuShaderModule;

#[derive(Debug, Clone)]
pub struct GpuShaderModule(WgpuShaderModule);

impl GpuShaderModule {
    pub fn new(value: WgpuShaderModule) -> Self {
        GpuShaderModule(value)
    }

    pub(crate) fn get_wgpu_shader_module(&self) -> &WgpuShaderModule {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct ShaderModule {
    value: GpuShaderModule,
}

impl ShaderModule {
    pub fn new(value: GpuShaderModule) -> Self {
        ShaderModule { value }
    }

    pub fn value(&self) -> &GpuShaderModule {
        &self.value
    }
}
