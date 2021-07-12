use bevy::{
    asset::{Assets, HandleUntyped},
    reflect::TypeUuid,
    render::{
        pipeline::{
            BlendComponent, BlendFactor, BlendOperation, BlendState, ColorTargetState, ColorWrite,
            CompareFunction, DepthBiasState, DepthStencilState, FrontFace, PipelineDescriptor,
            PolygonMode, PrimitiveState, PrimitiveTopology, StencilFaceState, StencilState,
        },
        shader::{Shader, ShaderStage, ShaderStages},
        texture::TextureFormat,
    },
};

pub const SPRITE_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 0xf8c045f774bd9729);

pub fn build_sprite_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    PipelineDescriptor {
        depth_stencil: Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::LessEqual,
            stencil: StencilState {
                front: StencilFaceState::IGNORE,
                back: StencilFaceState::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
            bias: DepthBiasState {
                constant: 0,
                slope_scale: 0.0,
                clamp: 0.0,
            },
        }),
        color_target_states: vec![ColorTargetState {
            blend: Some(BlendState {
                alpha: BlendComponent {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::Add,
                },
                color: BlendComponent {
                    src_factor: BlendFactor::SrcAlpha,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,
                },
            }),
            format: TextureFormat::default(),
            write_mask: ColorWrite::ALL,
        }],
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: PolygonMode::Fill,
            clamp_depth: false,
            conservative: false,
        },
        ..PipelineDescriptor::new(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("sprite.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("sprite.frag"),
            ))),
        })
    }
}

pub mod node {
    pub const SPRITE: &str = "sprite_x";
    pub const SPRITE_INSTANCE: &str = "sprite_x_instance";
}
