using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using OpenGL.Core;

namespace Nubila
{
    internal class Texture1D : Texture
    {
        protected int m_width;

        /// <summary>
        /// Create an empty texture of specific dimension
        /// </summary>
        /// <param name="width"></param>
        public Texture1D(
            int width
        ) : base(glTextureTarget.TEXTURE_1D, glInternalFormat.RGBA32F)
        {
            m_width = width;
            m_texture = gl.GenTexture();

            gl.ActiveTexture(glTextureUnit.TEXTURE0);
            gl.BindTexture(glTextureTarget.TEXTURE_1D, m_texture);

            float[] data = new float[m_width * 4];
            gl.TexImage1D(glTexture1DProxyTarget.TEXTURE_1D, 0, glInternalFormat.RGBA32F, m_width, 0, glPixelFormat.RGBA, glPixelDataType.FLOAT, data);
            gl.GenerateMipmap(glTextureTarget.TEXTURE_1D);

            SetFilterNearest();
            gl.TexParameteri(glTextureTarget.TEXTURE_1D, glTextureParameter.TEXTURE_WRAP_S, glTextureWrapMode.CLAMP_TO_EDGE);
            gl.TexParameteri(glTextureTarget.TEXTURE_1D, glTextureParameter.TEXTURE_WRAP_T, glTextureWrapMode.CLAMP_TO_EDGE);
        }

        /// <summary>
        /// Create a texture of specific dimension containing specific data
        /// </summary>
        /// <param name="width"></param>
        /// <param name="data"></param>
        /// <param name="format"></param>
        public Texture1D(
            int width,
            float[] data,
            glInternalFormat format
        ) : base(glTextureTarget.TEXTURE_1D, format)
        {
            m_width = width;
            m_texture = gl.GenTexture();

            gl.ActiveTexture(glTextureUnit.TEXTURE0);
            gl.BindTexture(glTextureTarget.TEXTURE_1D, m_texture);

            gl.TexImage1D(glTexture1DProxyTarget.TEXTURE_1D, 0, glInternalFormat.RGBA32F, m_width, 0, glPixelFormat.RGBA, glPixelDataType.FLOAT, data);
            gl.GenerateMipmap(glTextureTarget.TEXTURE_1D);

            SetFilterNearest();
            gl.TexParameteri(glTextureTarget.TEXTURE_1D, glTextureParameter.TEXTURE_WRAP_S, glTextureWrapMode.CLAMP_TO_EDGE);
            gl.TexParameteri(glTextureTarget.TEXTURE_1D, glTextureParameter.TEXTURE_WRAP_T, glTextureWrapMode.CLAMP_TO_EDGE);
        }
    }
}
