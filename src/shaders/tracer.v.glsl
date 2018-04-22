#version 330 core

in vec3 a_Pos;
out vec4 v_Color;
out vec2 fragCoord;

void main() {
  fragCoord = vec2(a_Pos.x, a_Pos.y);
  gl_Position = vec4(a_Pos, 1.0);
}