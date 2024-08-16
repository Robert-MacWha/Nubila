using OpenTK.Windowing.Common;
using OpenTK.Windowing.Desktop;
using OpenTK.Graphics.OpenGL4;
using OpenTK.Mathematics;

namespace Nubila
{
    static class DisplayManager
    {
        public static OpenTK.Windowing.Desktop.GameWindow window;
        public static Vector2 windowSize;
        public static float aspectRatio;

        public static void CreateWindow(int width = 800, int height = 600, string title = "New Window", int vsync = 1)
        {
            windowSize = new Vector2(width, height);
            aspectRatio = (float)width / (float)height;

            // Create a new window using OpenTK
            var nativeWindowSettings = new NativeWindowSettings()
            {
                Size = new Vector2i(width, height),
                Title = title,
                WindowBorder = WindowBorder.Fixed, // Fixed to make it non-resizable
            };

            window = new GameWindow(GameWindowSettings.Default, nativeWindowSettings);

            window.VSync = (VSyncMode)vsync; // Setting VSync

            window.Load += OnLoad;
            window.RenderFrame += OnRenderFrame;
            window.Resize += OnResize;

            window.Run(); // Run the window
        }

        private static void OnLoad()
        {
            // Set the clear color and viewport size
            GL.ClearColor(0, 0, 0, 0);
            GL.Viewport(0, 0, (int)windowSize.X, (int)windowSize.Y);
        }

        private static void OnRenderFrame(FrameEventArgs e)
        {
            PreRender();
            // Here, you would add your rendering code
            PostRender();

            window.SwapBuffers();
        }

        private static void OnResize(ResizeEventArgs e)
        {
            windowSize = new Vector2(e.Width, e.Height);
            aspectRatio = (float)e.Width / e.Height;
            GL.Viewport(0, 0, e.Width, e.Height);
        }

        public static void PreRender()
        {
            GL.Clear(ClearBufferMask.ColorBufferBit);
        }

        public static void PostRender()
        {
            // SwapBuffers is handled automatically by the event handler in OpenTK
        }

        public static void DestroyWindow()
        {
            window.Close();
            window.Dispose();
        }
    }
}
