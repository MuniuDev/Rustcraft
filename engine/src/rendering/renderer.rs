
use vulkano::device::Device;
use vulkano::framebuffer::{Subpass, RenderPassAbstract};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::GraphicsPipelineAbstract;

use std::sync::Arc;
use std::vec::Vec;

use crate::rendering::common::*;
use crate::rendering::vertex::Vertex;

pub struct Renderer {
    pub pipeline : Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
    render_pass : Arc<dyn RenderPassAbstract + Send + Sync>,
}

impl Renderer {
    pub fn new(device : Arc<Device>, render_pass : Arc<dyn RenderPassAbstract + Send + Sync>) -> Self {
        mod vs {
            vulkano_shaders::shader!{
                ty: "vertex",
                src: "
                    #version 450
                    layout(location = 0) in vec3 position;
                    void main() {
                        gl_Position = vec4(position, 1.0);
                    }
                "
            }
        }
    
        mod fs {
            vulkano_shaders::shader!{
                ty: "fragment",
                src: "
                    #version 450
                    layout(location = 0) out vec4 f_color;
                    void main() {
                        f_color = vec4(1.0, 0.0, 0.0, 1.0);
                    }
                "
            }
        }
    
        let vs = vs::Shader::load(device.clone()).unwrap();
        let fs = fs::Shader::load(device.clone()).unwrap();

        let pipeline = Arc::new(GraphicsPipeline::start()
        // We need to indicate the layout of the vertices.
        // The type `SingleBufferDefinition` actually contains a template parameter corresponding
        // to the type of each vertex. But in this code it is automatically inferred.
        .vertex_input_single_buffer::<Vertex>()
        // A Vulkan shader can in theory contain multiple entry points, so we have to specify
        // which one. The `main` word of `main_entry_point` actually corresponds to the name of
        // the entry point.
        .vertex_shader(vs.main_entry_point(), ())
        // The content of the vertex buffer describes a list of triangles.
        .triangle_list()
        // Use a resizable viewport set to draw over the entire window
        .viewports_dynamic_scissors_irrelevant(1)
        // See `vertex_shader`.
        .fragment_shader(fs.main_entry_point(), ())
        // We have to indicate which subpass of which render pass this pipeline is going to be used
        // in. The pipeline will only be usable from this particular subpass.
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        // Now that our builder is filled, we call `build()` to obtain an actual pipeline.
        .build(device.clone())
        .unwrap());

        return Renderer {
            pipeline,
            render_pass
        };
    }
}