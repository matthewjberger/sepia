#version 330 core
out vec4 FragColor;

in vec2 TexCoord;

uniform sampler2D tex;

const float offset = 1 / 300.0;

void main()
{

  // Normal
  // FragColor = texture(tex, TexCoord);

  // Inversion
  // FragColor = vec4(vec3(1.0 - texture(tex, TexCoord)), 1.0);

  // Grayscale (unweighted)
  // FragColor = texture(tex, TexCoord);
  // float average = (FragColor.r + FragColor.g + FragColor.b) / 3.0;
  // FragColor = vec4(average, average, average, 1.0);

  // Grayscale (weighted, more physically accurate)
  // FragColor = texture(tex, TexCoord);
  // float average = 0.2126 * FragColor.r + 0.7152 * FragColor.g + 0.0722 * FragColor.b;
  // FragColor = vec4(vec3(average).xyz, 1.0);

  // Applying a kernel
  vec2 offsets[9] = vec2[](
                           vec2(-offset,  offset), // top-left
                           vec2( 0.0f,    offset), // top-center
                           vec2( offset,  offset), // top-right
                           vec2(-offset,  0.0f),   // center-left
                           vec2( 0.0f,    0.0f),   // center-center
                           vec2( offset,  0.0f),   // center-right
                           vec2(-offset, -offset), // bottom-left
                           vec2( 0.0f,   -offset), // bottom-center
                           vec2( offset, -offset)  // bottom-right
                           );

  // Sharpen kernel
  float kernel[9] = float[](
                            -1, -1, -1,
                            -1,  9, -1,
                            -1, -1, -1
                            );

  // Blur kernel
  // float kernel[9] = float[](
  //                           1.0 / 16, 2.0 / 16, 1.0 / 16,
  //                           2.0 / 16, 4.0 / 16, 2.0 / 16,
  //                           1.0 / 16, 2.0 / 16, 1.0 / 16
  //                           );

  // Edge Detection kernel
  // float kernel[9] = float[](
  //                           1,  1, 1,
  //                           1, -8, 1,
  //                           1,  1, 1
  //                           );

  vec3 sampleTex[9];
  for(int i = 0; i < 9; i++)
  {
    sampleTex[i] = vec3(texture(tex, TexCoord.st + offsets[i]));
  }
  vec3 col = vec3(0.0);
  for(int i = 0; i < 9; i++)
    col += sampleTex[i] * kernel[i];

  FragColor = vec4(col, 1.0);
}
