using System.Numerics;

namespace Nubila
{
    internal class Camera
    {
        public static Vector3 Up { get; } = new Vector3(0, 1, 0);

        private Vector3 m_position;
        private Vector3 m_direction;
        private float m_fov;
        private float m_aspectRatio;

        public Matrix4x4 m_projectionMatrix;
        public Matrix4x4 m_viewMatrix;

        public Camera(float fov, float aspectRatio)
        {
            m_fov = (float)Math.PI * fov / 180.0f;
            m_aspectRatio = aspectRatio;

            m_position = new Vector3(5, 1, -5);
            m_direction = new Vector3(0, 0, -1);
            RefreshMatrix();
        }

        public void SetPosition(Vector3 position)
        {
            m_position = position;
            RefreshMatrix();
        }

        public void SetDirection(Vector3 direction)
        {
            m_direction = direction.Normalized();
            RefreshMatrix();
        }

        public void Translate(Vector3 translation)
        {
            m_position += translation;
            RefreshMatrix();
        }

        public Vector3 Position()
        {
            return m_position;
        }

        public Vector3 Direction()
        {
            return m_direction;
        }

        // ref: https://learn.microsoft.com/en-us/dotnet/api/system.numerics.matrix4x4?view=net-7.0
        private void RefreshMatrix()
        {
            m_projectionMatrix = Matrix4x4.CreatePerspectiveFieldOfView(m_fov, m_aspectRatio, 0.01f, 1000.0f);
            m_viewMatrix = Matrix4x4.CreateLookAt(m_position, m_position + m_direction, Up);
        }
    }
}
