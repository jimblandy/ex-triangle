#version 150

uniform vec3 screen_to_plane;
in vec2 position;

void main() {
  gl_Position = vec4(screen_to_plane.xy * position, 0.0, 1.0);
}
