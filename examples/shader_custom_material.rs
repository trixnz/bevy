use bevy::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup)
        .run();
}

#[derive(Uniforms, Default)]
struct MyMaterial {
    pub color: Color,
}

fn add_shader_to_render_graph(resources: &mut Resources) {
    let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();
    let mut pipelines = resources
        .get_mut::<AssetStorage<PipelineDescriptor>>()
        .unwrap();
    let mut shaders = resources.get_mut::<AssetStorage<Shader>>().unwrap();

    render_graph
        .build(&mut pipelines, &mut shaders)
        .add_resource_provider(UniformResourceProvider::<MyMaterial>::new(true))
        .add_pipeline_to_pass(resource_name::pass::MAIN, "MyMaterial", |builder| {
            builder
                .with_vertex_shader(Shader::from_glsl(
                    ShaderStage::Vertex,
                    r#"
                    #version 450
                    layout(location = 0) in vec3 Vertex_Position;
                    layout(set = 0, binding = 0) uniform Camera {
                        mat4 ViewProj;
                    };
                    layout(set = 1, binding = 0) uniform Object {
                        mat4 Model;
                    };
                    void main() {
                        gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
                    }
                "#,
                ))
                .with_fragment_shader(Shader::from_glsl(
                    ShaderStage::Fragment,
                    r#"
                    #version 450
                    layout(location = 0) out vec4 o_Target;
                    layout(set = 1, binding = 1) uniform MyMaterial_color {
                        vec4 color;
                    };
                    void main() {
                        o_Target = color;
                    }
                "#,
                ))
                .with_default_config();
        });
}

fn setup(world: &mut World, resources: &mut Resources) {
    // add our shader to the render graph
    add_shader_to_render_graph(resources);

    // create materials
    let mut material_storage = AssetStorage::<MyMaterial>::new();
    let material = material_storage.add(MyMaterial {
        color: Color::rgb(0.0, 0.8, 0.0),
    });
    resources.insert(material_storage);

    // get a handle to our newly created shader pipeline
    let mut pipeline_storage = resources
        .get_mut::<AssetStorage<PipelineDescriptor>>()
        .unwrap();
    let pipeline_handle = pipeline_storage.get_named("MyMaterial").unwrap();

    let mut mesh_storage = resources.get_mut::<AssetStorage<Mesh>>().unwrap();
    let cube_handle = mesh_storage.add(Mesh::from(shape::Cube));

    world
        .build()
        // cube
        .add_entity(MeshMaterialEntity::<MyMaterial> {
            mesh: cube_handle,
            renderable: Renderable {
                pipelines: vec![pipeline_handle],
                ..Default::default()
            },
            material,
            translation: Translation::new(0.0, 0.0, 0.0),
            ..Default::default()
        })
        // camera
        .add_entity(CameraEntity {
            local_to_world: LocalToWorld(Mat4::look_at_rh(
                Vec3::new(3.0, 8.0, 5.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            )),
            ..Default::default()
        });
}