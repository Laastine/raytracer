#version 330 core

in vec3 a_Pos;
out vec4 v_Color;

void main() {
  gl_Position = vec4(a_Pos, 1.0);
}