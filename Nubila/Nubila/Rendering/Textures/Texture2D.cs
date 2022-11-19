using System.Drawing;
using OpenGL.Core;

namespace Nubila
{
    internal class Texture2D : Texture
    {
        protected int m_width;
        protected int m_height;

        /// <summary>
        /// Create a empty texture of specified dimensions
        /// </summary>
        /// <param name="width"></param>
        /// <param name="height"></param>
        public Texture2D(int width, int height, glInternalFormat format) : base(glTextureTarget.TEXTURE_2D, format)
        {
            m_width = width;
            m_height = height;
            m_texture = gl.GenTexture();

            gl.ActiveTexture(glTextureUnit.TEXTURE0);
            gl.BindTexture(glTextureTarget.TEXTURE_2D, m_texture);

            float[] data = new float[m_width * m_height * 4];
            gl.TexImage2D(glTexture2DProxyTarget.TEXTURE_2D, 0, m_format, m_width, m_height, 0, glPixelFormat.RGBA, glPixelDataType.FLOAT, data);
            gl.GenerateMipmap(glTextureTarget.TEXTURE_2D);
            
            SetFilterLinear();
            gl.TexParameteri(glTextureTarget.TEXTURE_2D, glTextureParameter.TEXTURE_WRAP_S, glTextureWrapMode.CLAMP_TO_EDGE);
            gl.TexParameteri(glTextureTarget.TEXTURE_2D, glTextureParameter.TEXTURE_WRAP_T, glTextureWrapMode.CLAMP_TO_EDGE);
        }

        /// <summary>
        /// Load a texture from a file
        /// </summary>
        /// <param name="file"></param>
        public Texture2D(string file) : base(glTextureTarget.TEXTURE_2D, glInternalFormat.RGBA32F)
        {
            m_texture = gl.GenTexture();
            gl.BindTexture(glTextureTarget.TEXTURE_2D, m_texture);

            Bitmap btm = new Bitmap(file);
            m_width = btm.Width;
            m_height = btm.Height;

            float[] data = new float[m_width * m_height * 4];
            for (int x = 0; x < m_width; x ++)
            {
                for (int y = 0; y < m_height; y ++)
                {
                    // invert the y coord because opengl expects y=1 to be at the bottom
                    int invert_y = m_height - (y + 1);

                    int index = (invert_y * m_width + x) * 4;
                    Color color = btm.GetPixel(x, y);
                    data[index + 0] = color.R / 255.0f;
                    data[index + 1] = color.G / 255.0f;
                    data[index + 2] = color.B / 255.0f;
                    data[index + 3] = color.A / 255.0f;
                }
            }

            gl.TexImage2D(glTexture2DProxyTarget.TEXTURE_2D, 0, m_format, m_width, m_height, 0, glPixelFormat.RGBA, glPixelDataType.FLOAT, data);
            gl.GenerateMipmap(glTextureTarget.TEXTURE_2D);
            SetFilterLinear();
        }
        
    }
}
