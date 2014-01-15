#version 330 core

in vec2 texcoord;
uniform sampler2D tex;
out vec3 color;

void main(void) {
    //vec2 flipped_texcoord = vec2(texcoord.x, 1.0 - texcoord.y);
    //color = texture(tex, flipped_texcoord).rgb;
    color = texture(tex, texcoord).rgb;
}
