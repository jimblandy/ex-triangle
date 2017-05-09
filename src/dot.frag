#version 150
in vec2 frag_position;
out vec4 color;

void main() {
  if (frag_position.x < 0.0)
    color = vec4(0.0, 0.349, 1.0, 1.0);
  else if (frag_position.y < 0.0)
    color = vec4(1.0, 1.0, 0.0, 1.0);
  else
    color = vec4(1.0, 0.0, 0.0, 1.0);
}
