use gfx;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 4] = "a_Position",
    }
    constant Locals {
        color: [f32; 4] = "u_Color",
    }
    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "b_Locals",
        //texture: gfx::TextureSampler<[f32; 4]> = "u_Texture",
        ocolor: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
        odepth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}


const VS: &'static [u8] = b"
    #version 150 core
    in vec4 a_Position;
    void main() {
        gl_Position = a_Position;
    }
";
const FS: &'static [u8] = b"
    #version 150 core
    out vec4 Target0;
    uniform b_Locals {
        vec4 u_Color;
    };
    void main() {
        Target0 = u_Color;
    }
";

#[derive(Eq, Hash, PartialEq)]
pub struct Handle(u32);

pub struct Context<D: gfx::Device, F> {
    pub device: D,
    pub factory: F,
    pub encoder: gfx::Encoder<D::Resources, D::CommandBuffer>,
    pub out_color: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
    pub out_depth: gfx::handle::DepthStencilView<D::Resources, DepthFormat>,
    pub p2: gfx::PipelineState<D::Resources, pipe::Meta>
}

impl<D: gfx::Device, F: gfx::traits::FactoryExt<D::Resources>> Context<D, F> {
    pub fn new(d: D, mut f: F, cb: D::CommandBuffer,
               oc: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
               od: gfx::handle::DepthStencilView<D::Resources, DepthFormat>)
               -> Self
    {
        let prog = f.link_program(VS, FS).unwrap();
        let p2 = f.create_pipeline_from_program(&prog, gfx::Primitive::TriangleList,
                                                      gfx::state::Rasterizer::new_fill(), pipe::new()).unwrap();
        Context {
            p2,
            device: d,
            factory: f,
            encoder: cb.into(),
            out_color: oc,
            out_depth: od,

        }
    }

    pub fn add_triangles(&mut self, vertices: &Vec<Vertex>, color: [f32;4]) {

        let (vbuf, slice) = self.factory.create_vertex_buffer_with_slice(vertices, ());
        let locals = self.factory.create_constant_buffer(1);
        self.encoder.update_constant_buffer(&locals, &Locals { color});
        let pso = pipe::Data {
            vbuf: vbuf,
            locals: locals,
            ocolor: self.out_color.clone(),
            odepth: self.out_depth.clone(),
        };
        self.encoder.draw(&slice, &self.p2, &pso);
    }
}