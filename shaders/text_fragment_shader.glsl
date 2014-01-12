#version 330 core

in vec2 texcoord;
out vec4 color;
uniform sampler2D font_texture;

void main(){
 	vec2 flipped_texcoord = vec2(texcoord.x, 1.0 - texcoord.y);
    color = texture(font_texture, flipped_texcoord);
}