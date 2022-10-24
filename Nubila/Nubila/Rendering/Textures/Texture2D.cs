using System.Drawing;

namespace Nubila
{
    internal class Texture2D
    {
        int m_width;
        int m_height;
        int m_channels;

        public Texture2D(int m_width, int m_height, int m_channels)
        {

        }

        public Texture2D(string file)
        {
            // src: https://stackoverflow.com/questions/7296534/how-to-read-an-image-file-to-a-byte
            Bitmap bitmap = new Bitmap(file);
        }
    }
}
