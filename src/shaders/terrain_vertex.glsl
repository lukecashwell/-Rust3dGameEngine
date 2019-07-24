#version 410 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 texture_coordinate;
layout (location = 2) in vec3 normal;

uniform mat4 transformationMatrix;
uniform mat4 projectionMatrix;
uniform mat4 cameraMatrix;

uniform vec3 lightLocation;

out vec2 tc;
out vec3 surfaceNormal;
out vec3 toLightVector;
out vec3 toCameraVector;
out vec3 worldLocation;

void main()
{
	vec4 worldPosition = transformationMatrix * vec4(position, 1.0);
	gl_Position =  (projectionMatrix * cameraMatrix * worldPosition);
	tc = texture_coordinate; 

	surfaceNormal = (transformationMatrix * vec4(-normal.x, -normal.y, -normal.z, 0.0)).xyz;
	vec3 lightLoc = lightLocation;
	lightLoc.y *= -1;
	toLightVector =  lightLoc - worldPosition.xyz;

	vec3 cameraLoc = (inverse(cameraMatrix) * vec4(0.0, 0.0, 0.0, 1.0)).xyz;
	cameraLoc.y *= -1;
	toCameraVector  = cameraLoc - worldPosition.xyz;
	worldLocation = worldPosition.xyz;
}
