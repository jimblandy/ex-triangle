#version 150

uniform vec2 screen_to_plane;
uniform float fold_radius;
in vec2 position;

void main() {
  vec2 plane = position;
  float len = length(plane);
  if (len < 0.001) {
    gl_Position = vec4(plane, 0.0, 1.0);
    return;
  }
  vec2 normal = normalize(plane);

  if (fold_radius <= len) {
    len -= fold_radius;
    if (0.1570 <= len) {
      len -= 0.1570;
      len = fold_radius - len;
    } else {
      len = fold_radius + sin(len * 20) / 20;
    }
  }

  gl_Position = vec4(screen_to_plane * (normal * len), 0.0, 1.0);
}
