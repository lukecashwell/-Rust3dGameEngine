#version 410 core

layout (location = 0) out vec4 col;

in vec3 surfaceNormal;
in vec3 toLightVector;
in vec3 toCameraVector;
in vec3 worldLocation;
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

	float slope = dot(unitNormal,vec3(0, -1, 0));

	float nDot1 = dot(unitNormal,unitLightVector);
	float brightness = max(nDot1, 0.4);
	vec4 diffuse = vec4((1 - (brightness - 1)*(brightness - 1)) * lightColor * (lightBrightness/lightDistance), 1.0);

	vec3 unitCameraVector = normalize(toCameraVector);
	vec3 lightDirection = -unitLightVector;
	vec3 reflectedLight = reflect(lightDirection, unitNormal)  + vec3(-1, -1, -1);

	float specularFactor = dot(vec3(1, 1, 1)-(reflectedLight*reflectedLight),unitCameraVector);
	specularFactor = max(specularFactor, 0.0);
	float dampedFactor = pow(specularFactor, shineDamper);
	vec3 finalSpecular = dampedFactor * reflectivity * lightColor * (lightBrightness/lightDistance);

	vec4 textureColor = texture(tex, tc);
	float average = (textureColor.x + textureColor.y + textureColor.z) / 3;
	vec4 rockColor = vec4(average, average, average, 1.0);

	float weight = min(-min((-50-worldLocation.y)/50,0),1);

	vec4 finalColor = vec4(weight/2, 0.0, 0.0, 1.0) + (textureColor*(slope) + rockColor*(1-slope));

 	col = diffuse * ( finalColor ) +  vec4(finalSpecular, 1.0);
}
