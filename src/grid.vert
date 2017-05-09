#version 150

uniform vec3 screen_to_plane;
uniform float fold_radius;
in vec2 position;

vec2 fold(vec2 v, float radius) {
  // Convert v to direction/length form. If we can't compute direction because
  // the line is too short, return the point unchanged.
  float len = length(v);
  if (len < 0.001) {
    return v;
  }

  vec2 direction = normalize(v);
  if (radius <= len) {
    len -= radius;              // distance beyond radius
    if (0.1570 <= len) {
      // v is past curved area, so proceeds straight down from its end
      len -= 0.1570;            // distance past curved area
      len = radius - len;
    } else {
      // v is within curved area, use sin to curve back downwards
      len = fold_radius + sin(len * 20) / 20;
    }
  }

  // Convert direction/length back to vector.
  return direction * len;
}

void main() {
  vec2 plane = position;

  plane = fold(plane, fold_radius);

  gl_Position = vec4(screen_to_plane.xy * plane, 0.0, 1.0);
}
