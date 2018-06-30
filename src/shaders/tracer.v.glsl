#version 330 core

in vec2 a_Pos;
out vec2 fragCoord;

void main() {
  fragCoord = a_Pos;
  gl_Position = vec4(a_Pos, 0.0, 1.0);
}