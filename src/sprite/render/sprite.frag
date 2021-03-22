#version 450

layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

// layout(set = 1, binding = 1) uniform SpriteInstance {
//     vec4 Color;
//     uint Flip;
// };

layout(set = 2, binding = 0) uniform Sprite_color_base {
    vec4 ColorBase;
};

#ifdef SPRITE_TEXTURE 
layout(set = 2, binding = 1) uniform texture2D Sprite_texture;
layout(set = 2, binding = 2) uniform sampler Sprite_texture_sampler;
#endif

void main() {
    // vec4 color = Color * ColorBase;

    vec4 color = ColorBase;
#ifdef SPRITE_TEXTURE
    color *= texture(sampler2D(Sprite_texture, Sprite_texture_sampler), v_Uv);
#endif
    o_Target = color;
}
