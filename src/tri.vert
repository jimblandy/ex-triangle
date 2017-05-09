#version 150

uniform vec3 screen_to_plane;
in vec2 position;
out vec2 frag_position;

void main() {
  frag_position = position;
  gl_Position = vec4(screen_to_plane.xy * position, 0.0, 1.0);
}
