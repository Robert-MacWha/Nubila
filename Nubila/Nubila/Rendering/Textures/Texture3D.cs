using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using OpenGL.Core;

namespace Nubila
{
    internal class Texture3D : Texture
    {
        protected int m_width;
        protected int m_height;
        protected int m_depth;

        /// <summary>
        /// Create an empty texture of specific dimensions
        /// </summary>
        /// <param name="width"></param>
        /// <param name="height"></param>
        /// <param name="depth"></param>
        /// <param name="format"></param>
        public Texture3D(
            int width, int height, int depth, 
            glInternalFormat format
        ) : base(glTextureTarget.TEXTURE_3D, format)
        {
            m_width = width;
            m_height = height;
            m_depth = depth;
            m_texture = gl.GenTexture();

            gl.ActiveTexture(glTextureUnit.TEXTURE0);
            gl.BindTexture(glTextureTarget.TEXTURE_3D, m_texture);

            float[] data = new float[m_width * m_height * m_depth * 4];
            gl.TexImage3D(glTexture3DProxyTarget.TEXTURE_3D, 0, m_format, m_width, m_height, m_depth, 0, glPixelFormat.RGBA, glPixelDataType.FLOAT, data);
            gl.GenerateMipmap(glTextureTarget.TEXTURE_3D);

            SetFilterNearest();
            gl.TexParameteri(glTextureTarget.TEXTURE_3D, glTextureParameter.TEXTURE_WRAP_S, glTextureWrapMode.CLAMP_TO_EDGE);
            gl.TexParameteri(glTextureTarget.TEXTURE_3D, glTextureParameter.TEXTURE_WRAP_T, glTextureWrapMode.CLAMP_TO_EDGE);
        }

        /// <summary>
        /// Create a texture of specific dimensions containing specific data
        /// </summary>
        /// <param name="width"></param>
        /// <param name="height"></param>
        /// <param name="depth"></param>
        /// <param name="data"></param>
        /// <param name="format"></param>
        public Texture3D(
            int width, int height, int depth, 
            float[] data,
            glInternalFormat format
        ) : base(glTextureTarget.TEXTURE_3D, format)
        {
            m_width = width;
            m_height = height;
            m_depth = depth;
            m_texture = gl.GenTexture();

            gl.ActiveTexture(glTextureUnit.TEXTURE0);
            gl.BindTexture(glTextureTarget.TEXTURE_3D, m_texture);

            gl.TexImage3D(glTexture3DProxyTarget.TEXTURE_3D, 0, m_format, m_width, m_height, m_depth, 0, glPixelFormat.RGBA, glPixelDataType.FLOAT, data);
            gl.GenerateMipmap(glTextureTarget.TEXTURE_3D);

            SetFilterNearest();
            gl.TexParameteri(glTextureTarget.TEXTURE_3D, glTextureParameter.TEXTURE_WRAP_S, glTextureWrapMode.CLAMP_TO_EDGE);
            gl.TexParameteri(glTextureTarget.TEXTURE_3D, glTextureParameter.TEXTURE_WRAP_T, glTextureWrapMode.CLAMP_TO_EDGE);
        }
    }
}
