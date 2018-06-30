#version 330 core

in vec2 fragCoord;
out vec4 Target0;

uniform b_Time {
  float u_Time;
};
uniform samplerCube t_Cubemap;

const vec3 cameraPos = vec3(0.0, 0.0, 20.0);
const int maxReflections = 20;
const int sphereCount = 4;
const float highValue = pow(10.0, 6.0);
const float lowValue = pow(10.0, -4.0);

void calcSphereData(vec3 spherePos, float sphereSize, vec3 rayDirection, vec3 rayOrigin, out float firstIntersectionPointOfSphere, out vec3 sphereNormal) {
  vec3 rayToSphere = spherePos - rayOrigin;
  float radius2 = pow(sphereSize, 2);
  float closestDistanceToSphereDiagonal = dot(rayToSphere, rayDirection);
  float d2 = dot(rayToSphere, rayToSphere) - closestDistanceToSphereDiagonal * closestDistanceToSphereDiagonal;

  if (d2 > radius2) {
    firstIntersectionPointOfSphere = -1.0;
    sphereNormal = vec3(0.0, 0.0, 0.0);
    return;
  }

  float distanceFromIntersectionToDiagonal = sqrt(radius2 - d2);
  firstIntersectionPointOfSphere = closestDistanceToSphereDiagonal - distanceFromIntersectionToDiagonal;

  if (firstIntersectionPointOfSphere < 0.0) {
    firstIntersectionPointOfSphere = closestDistanceToSphereDiagonal + distanceFromIntersectionToDiagonal;
  }

  sphereNormal = normalize(rayOrigin + (firstIntersectionPointOfSphere * rayDirection) - spherePos);
}

void rayToScene(vec3 spheres[sphereCount],
  vec3 sphereColors[sphereCount],
  float sphereSizes[sphereCount],
  vec3 ray,
  vec3 rayOrigin,
  out vec3 rayColor) {
  rayColor = vec3(1.0, 1.0, 1.0);

  for (int currentReflection = 0; currentReflection < maxReflections + 1; currentReflection++) {
    vec3 foundColor = vec3(1.0, 1.0, 1.0);
    vec3 reflectionNormal = vec3(0.0, 0.0, 0.0);
    float depthValue = highValue;

    for (int idx = 0; idx < sphereCount; idx++) {
      vec3 sphereNormal;
      float firstIntersectionPoint;

      calcSphereData(spheres[idx], sphereSizes[idx], ray, rayOrigin, firstIntersectionPoint, sphereNormal);

      if (firstIntersectionPoint > lowValue && firstIntersectionPoint < depthValue) {
        depthValue = firstIntersectionPoint;
        reflectionNormal = sphereNormal;
        foundColor = sphereColors[idx];
      }
    }

    rayColor = foundColor * rayColor;

    if (depthValue > highValue || currentReflection == maxReflections) {
      rayColor = texture(t_Cubemap, ray).rgb * rayColor;
      break;
    }

    rayOrigin += ray * depthValue;

    ray = reflect(ray, reflectionNormal);
  }
}

void generateSpheres(out vec3[sphereCount] spheres, out vec3[sphereCount] sphereColors, out float[sphereCount] sphereSizes) {
  float cosTime = cos(u_Time);
  float sinTime = sin(u_Time);
  float cosTimeA = cos(u_Time * 0.25);
  float sinTimeB = sin(u_Time * 0.25);
  float sinTimeC = sin(u_Time * 1.25);

  mat3 rotation = mat3(vec3(1.0, 0.0, 0.0),            vec3(0.0, cosTimeA, -sinTimeB), vec3(0.0, sinTimeB, cosTimeA))
                * mat3(vec3(cosTime, 0.0, sinTime),    vec3(0.0, 1.0, 0.0),            vec3(-sinTime, 0.0, cosTime))
                * mat3(vec3(cosTimeA, -sinTimeB, 0.0), vec3(sinTimeB, cosTimeA, 0.0),  vec3(0.0, 0.0, 1.0));

  spheres = vec3[sphereCount](vec3(-7.5, -7.5, sinTime),
                              vec3(7.5, -7.5, sinTimeB),
                              vec3(0.0, 7.5 * (sinTimeC + 5.0) * 0.25, sinTimeC),
                              vec3(7.5 * (sinTimeC - 3.0) * 0.5, 5.0, sinTimeC));

  sphereColors = vec3[sphereCount](vec3(1.0, 0.5, 0.5),
                                    vec3(0.5, 1.0, 0.5),
                                    vec3(0.5, 0.5, 1.0),
                                    vec3(0.8, 0.8, 0.8));

  sphereSizes = float[sphereCount](6.0, 4.0, 3.5, 4.5);

  for (int idx = 0; idx < sphereCount; idx++) {
    spheres[idx] = rotation * spheres[idx] + cameraPos;
  }
}

void main() {
  vec3 ray = normalize(vec3(fragCoord, 0.5));
  vec3 rayColor;
  vec3 rayOrigin = vec3(fragCoord, 0.0);
  vec3 spheres[sphereCount];
  vec3 sphereColors[sphereCount];
  float sphereSizes[sphereCount];

  generateSpheres(spheres, sphereColors, sphereSizes);

  rayToScene(spheres, sphereColors, sphereSizes, ray, rayOrigin, rayColor);

  Target0 = vec4(rayColor, 1.0);
}