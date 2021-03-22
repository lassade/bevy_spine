#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec2 Vertex_Uv;

layout(location = 0) out vec2 v_Uv;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

layout(set = 2, binding = 0) uniform Transform {
    mat4 Model;
};
// layout(set = 2, binding = 1) uniform SpriteInstance {
//     vec4 Color;
//     uint Flip;
// };

void main() {
    vec3 position = Vertex_Position.xyz;
    // if ((Flip & 1) != 0) {
    //     position.x = -position.x;
    // }
    // if ((Flip & 2) != 0) {
    //     position.y = -position.y;
    // }

    v_Uv = Vertex_Uv;
    gl_Position = ViewProj * Model * vec4(position, 1.0);
}