#version 330 core

// Input vertex attributes
layout (location = 0) in vec2 aPos;           // Vertex position
layout (location = 1) in vec2 aTexCoord;       // Texture coordinates
layout (location = 2) in vec2 aOffset;         // Instance offset
layout (location = 3) in float aTexIndex;      // Texture index
layout (location = 4) in float aHighlight;     // Highlight flag
layout (location = 5) in float aSelectedTile;  // Selected tile flag
layout (location = 6) in float aResource;      // Resource flag
layout (location = 7) in float aWorker;        // Worker flag
layout (location = 8) in float aTroops;        // Troops flag
layout (location = 9) in float aStructure;     // Structure flag
layout (location = 10) in float aReservedFloat; // Reserved attribute for fruture use
layout (location = 11) in float aGridEnable;   // Grid enable flag
layout (location = 12) in float aHighlightResources; // Highlight all resources flag
layout (location = 13) in vec4 aHighlightColor; // Highlight color for tile
layout (location = 14) in float aTextureDimensions; // Dimensions of the texture
layout (location = 15) in float aTextureIndex; // Index of texture subdivision

// Output to fragment shader
out vec2 TexCoord;
flat out float TexIndex;
out float Highlight;
out float SelectedTile;
out float Resource;
out float Worker;
out float Troops;
out float Structure;
out float ReservedFloat;
out float GridEnable;
out float HighlightResources;
out vec4 HighlightColor;
out float TextureDimensions;
out float TextureIndex;
out vec2 FragPos;
out vec3 Normal;
out float IsVisible;  // Visibility flag

// Uniform variables
uniform mat4 projection;       // Projection matrix
uniform vec2 screenDimensions; // Screen width and height
uniform vec2 cameraOffset;     // Camera's current position
uniform float panningRotation; // Rotation angle for 3D-like effect
uniform float gameTime;        // Game time (optional)
uniform float textureSize;     //Defines size of the tile

// Rotation matrix creation function
mat2 rotationMatrix(float angle) {
    float s = sin(radians(angle));
    float c = cos(radians(angle));
    return mat2(c, -s, s, c);
}

void main()
{
    // Calculate world position with 3D-like rotation
    vec2 screenCenter = screenDimensions * 0.5;
    vec2 offsetFromCenter = aOffset - screenCenter;
    
    // Apply rotation matrix
    mat2 rotMat = rotationMatrix(panningRotation);
    vec2 rotatedFromCenter = rotMat * offsetFromCenter;
    
    // Combine rotation back to original coordinate system
    vec2 finalOffset = rotatedFromCenter + screenCenter;
    
    // Calculate world position 
    vec2 worldPos = aPos + finalOffset;
    
    // Project the world position
    gl_Position = projection * vec4(worldPos, 0.0, 1.0);
    
    // Culling logic with buffer zones
    vec2 screenPos = worldPos - cameraOffset;
    
    float bufferX = textureSize * 2.0;
    float bufferY = textureSize * 2.0;
    
    // Determine tile visibility
    bool xVisible = screenPos.x + textureSize > -bufferX && 
                    screenPos.x < screenDimensions.x + bufferX;
    bool yVisible = screenPos.y + textureSize > -bufferY && 
                    screenPos.y < screenDimensions.y + bufferY;
    
    IsVisible = (xVisible && yVisible) ? 1.0 : 0.0;
    
    // Pass through other attributes
    TexCoord = aTexCoord;
    TexIndex = aTexIndex;
    Highlight = aHighlight;
    SelectedTile = aSelectedTile;
    Resource = aResource;
    Worker = aWorker;
    Troops = aTroops;
    Structure = aStructure;
    ReservedFloat = aReservedFloat;
    GridEnable = aGridEnable;
    HighlightResources = aHighlightResources;
    HighlightColor = aHighlightColor;
    TextureDimensions = aTextureDimensions;
    TextureIndex = aTextureIndex;

    FragPos = aPos;
    Normal = vec3(0.0, 0.0, 1.0);
}