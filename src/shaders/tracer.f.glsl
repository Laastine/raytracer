#version 330 core

uniform b_Time {
  float iTime;
};

in vec2 fragCoord;
out vec4 Target0;

void main() {
  vec2 iResolution = vec2(1280.0, 720.0);
  vec2 uv = fragCoord / iResolution.xy;
  vec3 col = 0.5 + 0.5 * cos(iTime+uv.xyx+vec3(0, 2, 4));
  Target0 = vec4(col, 1.0);
}