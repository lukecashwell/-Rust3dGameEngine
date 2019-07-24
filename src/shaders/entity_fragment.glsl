#version 410 core

layout (location = 0) out vec4 col;

in vec3 surfaceNormal;
in vec3 toLightVector;
in vec3 toCameraVector;
in vec2 tc;

uniform sampler2D tex;
uniform vec3 lightColor;
uniform float lightBrightness;

uniform float shineDamper;
uniform float reflectivity;

void main()
{
	vec3 unitNormal = normalize(surfaceNormal);
    vec3 unitLightVector = normalize(toLightVector);
    float lightDistance = toLightVector.x*toLightVector.x + toLightVector.y*toLightVector.y + toLightVector.z*toLightVector.z;

	float nDot1 = dot(unitNormal,unitLightVector);
	float brightness = max(nDot1, 0.20);
	vec3 diffuse = (1 - (brightness - 1)*(brightness - 1)) * lightColor * (lightBrightness/lightDistance);

	vec3 unitCameraVector = normalize(toCameraVector);
	vec3 lightDirection = -unitLightVector;
	vec3 reflectedLight = reflect(lightDirection, unitNormal);

	float specularFactor = dot(reflectedLight,unitCameraVector);
	specularFactor = max(specularFactor, 0.0);
	float dampedFactor = pow(specularFactor, shineDamper);
	vec3 finalSpecular = dampedFactor * reflectivity * lightColor * ((lightBrightness)/lightDistance);

	vec4 textureColor = texture(tex, tc);
	if (textureColor.a < 0.5) {
		discard;
	}

 	col = vec4(diffuse, 1.0) * texture(tex, tc) +  vec4(finalSpecular, 1.0);
}
