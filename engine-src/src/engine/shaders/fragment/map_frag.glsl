#version 330 core
out vec4 FragColor;

in vec2 TexCoord;
flat in float TexIndex;
in float Highlight;
in float SelectedTile;
in float Resource;
in float Worker;
in float Troops;
in float Structure;
in float ReservedFloat;
in float GridEnable;
in float HighlightResources;
in vec4 HighlightColor;
in float TextureDimensions;
in float TextureIndex;
in vec2 FragPos;
in vec3 Normal;

// Texture atlas uniform
uniform sampler2D textureatlas;

// New lighting uniforms
uniform float gameTime;  // Time in seconds (0-4500 for full day cycle)
uniform float dayDuration;  // Total duration of day+night cycle in seconds
uniform vec2 screenDimensions;  // Screen width and height

//Texture Related Uniforms
uniform float textureSize; //Defines size of the tile
uniform vec2 textureAtlasSize; // Size of the texture atlas

uniform float ticks;

vec3 calculateSunPosition() {
     // Calculate day and night portions
     float nightPortion = 0.2;  // 20% of total time is night
     float dayLength = dayDuration * (1.0 - nightPortion);    // 80% of total time
     float nightLength = dayDuration * nightPortion;          // 20% of total time
     
     // Offset the sun angle to start at 10 AM (approximately 1/3 into the day)
     float morningOffset = 1.047;  // PI/3 radians
     
     // Calculate sun angle based on time of day
     float sunAngle;
     if (mod(gameTime, dayDuration) < dayLength) {
          // During day (80% of total time)
          sunAngle = (mod(gameTime, dayLength) / dayLength) * 3.14159 + morningOffset;
     } else {
          // During night (20% of total time)
          sunAngle = 3.14159 + ((mod(gameTime, dayLength) / nightLength) * 3.14159) + morningOffset;
     }
     
     float sunHeight = sin(sunAngle);
     float sunX = cos(sunAngle);
     
     return normalize(vec3(sunX, sunHeight, 0.5));
}

vec3 calculateLighting(vec4 baseColor) {
     vec3 sunPosition = calculateSunPosition();
     
     float ambientStrength = 0.2;
     vec3 ambient = ambientStrength * vec3(1.0);
     
     float diff = max(dot(Normal, sunPosition), 0.0);
     vec3 diffuse = diff * vec3(1.0);
     
     float nightPortion = 0.2;
     float dayLength = dayDuration * (1.0 - nightPortion);
     float timeInCycle = mod(gameTime, dayDuration);
     
     float intensityMultiplier;
     if (timeInCycle < dayLength) {
          // During day
          float dayProgress = (timeInCycle / dayLength + 1.0/3.0);
          if (dayProgress > 1.0) dayProgress -= 1.0;
          
          float noonBoost = 1.25;
          float noonFocus = 3.0;
          intensityMultiplier = max(pow(sin(dayProgress * 3.14159), noonFocus) * noonBoost + 0.3, 0.3);
     } else {
          // During night
          intensityMultiplier = 0.3;
     }
     
     return (ambient + diffuse) * intensityMultiplier;
}

void highlight_Frag(float minDist, vec4 highlightColor, float borderThickness, vec4 texColor) {
     if (minDist < borderThickness) {
          FragColor = mix(texColor, highlightColor, 0.9);
     }
}

void resource_Frag(float distFromCenter, vec4 texColor) {
     vec4 resourceColor;

     if (Resource == 1.0) {
          resourceColor = vec4(0.75, 0.75, 0.75, 1.0);
     } else if (Resource == 2.0) {
          resourceColor = vec4(0.54, 0.27, 0.07, 1.0);
     } else if (Resource == 3.0) {
          resourceColor = vec4(1.0, 0.27, 0.0, 1.0);
     }

     if (distFromCenter < 1.0) {
          FragColor = mix(texColor, resourceColor, 0.9);
     }
}

void overlayTexture(vec4 baseColor, vec4 overlayColor, float overlayActive) {
     if (overlayActive > 0.5) {
          FragColor = mix(baseColor, overlayColor, overlayColor.a);
     }
}

vec2 getCoordsFromAtlas(int xIndex, int yIndex, float fDimensions, float fSubIndex) {
     int dimensions = int(fDimensions);
     int subIndex = int(fSubIndex);

     float tileWidth = textureSize / textureAtlasSize.x;
     float tileHeight = textureSize / textureAtlasSize.y;

     float tileUMin = xIndex * tileWidth;
     float tileVMin = yIndex * tileHeight;
     float tileUMax = tileUMin + tileWidth;
     float tileVMax = tileVMin + tileHeight;

     if (dimensions > 1) {
          int subXIndex = subIndex % dimensions;
          int subYIndex = subIndex / dimensions;

          float subTileWidth = tileWidth / float(dimensions);
          float subTileHeight = tileHeight / float(dimensions);

          float subUMin = tileUMin + subXIndex * subTileWidth;
          float subUMax = subUMin + subTileWidth;
          float subVMin = tileVMin + subYIndex * subTileHeight;
          float subVMax = subVMin + subTileHeight;

          vec2 texCoords = vec2(
               mix(subUMin, subUMax, TexCoord.x),
               mix(subVMin, subVMax, TexCoord.y)
          );

          return texCoords;
     } else {
          vec2 texCoords = vec2(
               mix(tileUMin, tileUMax, TexCoord.x),
               mix(tileVMin, tileVMax, TexCoord.y)
          );

          return texCoords;
     }
}

void main()
{
     vec4 texColor;
     vec4 highlightColor = vec4(0.0, 1.0, 0.0, 1.0);
     vec4 selectedColor = vec4(1.0, 1.0, 0.0, 1.0);
     vec4 gridColor = vec4(0.0, 0.0, 0.0, 1.0);
     vec4 resourceHighlightColor = vec4(1.0, 0.0, 0.0, 1.0);

     float borderThickness = max(2.0, textureSize / 16.0);
     vec2 distFromEdge = min(FragPos, vec2(textureSize) - FragPos);
     float minDist = min(distFromEdge.x, distFromEdge.y);

     float distFromCenter = length(FragPos - vec2(textureSize / 2.0, textureSize / 2.0)) / (textureSize / 8.0);

     // Select base texture
     if (TexIndex == 0.0) {
          texColor = texture(textureatlas, getCoordsFromAtlas(0, 0, 1, 1));
     } else if (TexIndex == 1.0) {
          texColor = texture(textureatlas, getCoordsFromAtlas(1, 0, 1, 1));
     } else if (TexIndex == 2.0) {
          texColor = texture(textureatlas, getCoordsFromAtlas(2, 0, 1, 1));
     } else if (TexIndex == 3.0) {
          texColor = texture(textureatlas, getCoordsFromAtlas(3, 0, 1, 1));
     } else {
          texColor = texture(textureatlas, getCoordsFromAtlas(4, 0, 1, 1));
     }

     // Start with base color and apply lighting
     FragColor = texColor;
     vec3 lighting = calculateLighting(FragColor);
     FragColor.rgb *= lighting;

     // Apply grid
     if (GridEnable > 0.5 && minDist < borderThickness) {
          FragColor = mix(FragColor, gridColor, 1.0);
     }

     // Apply resource effect
     if (Resource > 0.5) {
          resource_Frag(distFromCenter, FragColor);
     }
     
     // Apply structure overlays
     if (Structure > 0.0) {
          if (Structure == 1.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(5, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 2.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(6, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 3.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(7, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 4.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(11, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 5.0) {
               if (int(ticks / 200) % 3 == 0) {
                    vec4 fortColor = texture(textureatlas, getCoordsFromAtlas(17, 1, TextureDimensions, TextureIndex));
                    overlayTexture(FragColor, fortColor, Structure);
               } else if (int(ticks / 200) % 3 == 1) {
                    vec4 fortColor = texture(textureatlas, getCoordsFromAtlas(18, 1, TextureDimensions, TextureIndex));
                    overlayTexture(FragColor, fortColor, Structure);
               } else {
                    vec4 fortColor = texture(textureatlas, getCoordsFromAtlas(19, 1, TextureDimensions, TextureIndex));
                    overlayTexture(FragColor, fortColor, Structure);
               }
          }
          else if (Structure == 6.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(10, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 7.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(9, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 8.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(12, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 9.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(13, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 10.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(14, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 11.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(15, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 12.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(16, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 13.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(17, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 14.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(18, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 15.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(19, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 16.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(20, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 17.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(21, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 18.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(22, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 19.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(23, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 20.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(24, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 21.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(25, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 22.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(26, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 23.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(27, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 24.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(58, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 25.0) {
               vec4 structureColor = texture(textureatlas, getCoordsFromAtlas(0, 1, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, structureColor, Structure);
          }
          else if (Structure == 26.0) {
               vec4 roadAllSidesColor = texture(textureatlas, getCoordsFromAtlas(1, 1, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, roadAllSidesColor, Structure);
          }
          else if (Structure == 27.0) {
               vec4 roadBottomLeftColor = texture(textureatlas, getCoordsFromAtlas(2, 1, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, roadBottomLeftColor, Structure);
          }
          else if (Structure == 28.0) {
               vec4 roadBottomColor = texture(textureatlas, getCoordsFromAtlas(3, 1, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, roadBottomColor, Structure);
          }
          else if (Structure == 29.0) {
               vec4 roadHorizontalColor = texture(textureatlas, getCoordsFromAtlas(4, 1, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, roadHorizontalColor, Structure);
          }
          else if (Structure == 30.0) {
               vec4 roadLeftColor = texture(textureatlas, getCoordsFromAtlas(5, 1, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, roadLeftColor, Structure);
          }
          else if (Structure == 31.0) {
               vec4 roadNoConnectionColor = texture(textureatlas, getCoordsFromAtlas(6, 1, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, roadNoConnectionColor, Structure);
          }
          else if (Structure == 32.0) {
               vec4 roadRightBottomLeftColor = texture(textureatlas, getCoordsFromAtlas(7, 1, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, roadRightBottomLeftColor, Structure);
          }
          else if (Structure == 33.0) {
               vec4 roadRightBottomColor = texture(textureatlas, getCoordsFromAtlas(8, 1, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, roadRightBottomColor, Structure);
          }
          else if (Structure == 34.0) {
               vec4 roadRightColor = texture(textureatlas, getCoordsFromAtlas(9, 1, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, roadRightColor, Structure);
          }
          else if (Structure == 35.0) {
               vec4 roadTopBottomLeftColor = texture(textureatlas, getCoordsFromAtlas(10, 1, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, roadTopBottomLeftColor, Structure);
          }
          else if (Structure == 36.0) {
                    vec4 roadTopLeftColor = texture(textureatlas, getCoordsFromAtlas(11, 1, TextureDimensions, TextureIndex));
                    overlayTexture(FragColor, roadTopLeftColor, Structure);
          }
          else if (Structure == 37.0) {
               vec4 roadTopColor = texture(textureatlas, getCoordsFromAtlas(12, 1, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, roadTopColor, Structure);
          }
          else if (Structure == 38.0) {
               vec4 roadTopRightBottomColor = texture(textureatlas, getCoordsFromAtlas(13, 1, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, roadTopRightBottomColor, Structure);
          }
          else if (Structure == 39.0) {
               vec4 roadTopRightLeftColor = texture(textureatlas, getCoordsFromAtlas(14, 1, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, roadTopRightLeftColor, Structure);
          }
          else if (Structure == 40.0) {
               vec4 roadTopRightColor = texture(textureatlas, getCoordsFromAtlas(15, 1, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, roadTopRightColor, Structure);
          }
          else if (Structure == 41.0) {
               vec4 roadVerticalColor = texture(textureatlas, getCoordsFromAtlas(16, 1, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, roadVerticalColor, Structure);
          }
     }
     
     // Apply worker overlay
     if (Worker == 1.0) {
          vec4 workerColor = texture(textureatlas, getCoordsFromAtlas(28, 0, TextureDimensions, TextureIndex));
          overlayTexture(FragColor, workerColor, Worker);
     }

     if (Troops == 1.0) {
          if (int(ticks / 200) % 3 == 0) {
               vec4 troopColor = texture(textureatlas, getCoordsFromAtlas(36, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, troopColor, Troops);
          } else if (int(ticks / 200) % 3 == 1) {
               vec4 troopColor = texture(textureatlas, getCoordsFromAtlas(37, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, troopColor, Troops);
          } else {
               vec4 troopColor = texture(textureatlas, getCoordsFromAtlas(38, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, troopColor, Troops);
          }
     }

     // Apply highlights
     if ((SelectedTile > 0.0) || (Highlight > 0.0)) {
          if ((SelectedTile > 0.0) && (Highlight > 0.0)) {
               vec4 resourceColor = texture(textureatlas, getCoordsFromAtlas(35, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, resourceColor, 1.0);
               // highlight_Frag(minDist, selectedColor, borderThickness, FragColor);
          }
          else if (SelectedTile > 0.0) {
               vec4 resourceColor = texture(textureatlas, getCoordsFromAtlas(35, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, resourceColor, 1.0);
               // highlight_Frag(minDist, selectedColor, borderThickness, FragColor);
          }
          else if (Highlight > 0.0) {
               vec4 resourceColor = texture(textureatlas, getCoordsFromAtlas(30, 0, TextureDimensions, TextureIndex));
               overlayTexture(FragColor, resourceColor, 1.0);
               // highlight_Frag(minDist, highlightColor, borderThickness, FragColor);
          }
     }

     // Apply Resource Highlight
     if ((HighlightResources > 0.0) && (Resource > 0.0)) {
          vec4 resourceColor = texture(textureatlas, getCoordsFromAtlas(34, 0, TextureDimensions, TextureIndex));
          overlayTexture(FragColor, resourceColor, 1.0);
     }

     // Apply Highlight Color
     if (HighlightColor.a > 0.0) {
          highlight_Frag(minDist, HighlightColor, borderThickness, FragColor);
     }
}