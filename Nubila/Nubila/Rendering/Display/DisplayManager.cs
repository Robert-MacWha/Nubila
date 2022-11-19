
using System.Numerics;
using GLFW;
using OpenGL.Core;

namespace Nubila
{
    static class DisplayManager
    {
        public static Window window;
        public static Vector2 windowSize;
        public static float aspectRatio;

        public static void CreateWindow(int width = 800, int height = 600, string title = "New Window", int vsync = 1)
        {
            windowSize = new Vector2(width, height);
            aspectRatio = (float)width / (float)height;

            // GLFW initialization
            Glfw.Init();
            Glfw.WindowHint(Hint.ContextVersionMajor, 4);
            Glfw.WindowHint(Hint.ContextVersionMinor, 3);
            Glfw.WindowHint(Hint.OpenglProfile, Profile.Core);

            Glfw.WindowHint(Hint.Focused, true);
            Glfw.WindowHint(Hint.Resizable, false);

            // create new window
            window = Glfw.CreateWindow(width, height, title, GLFW.Monitor.None, Window.None);
            if (window == Window.None)
            {
                // window creation failed
                return;
            }
            Glfw.MakeContextCurrent(window);

            gl.LoadOpenGL();
            gl.Viewport(0, 0, width, height);
            Glfw.SwapInterval(vsync);
        }

        public static void PreRender()
        {
            gl.ClearColor(0, 0, 0, 0);
            gl.Clear(glBufferMask.COLOR_BUFFER_BIT);
        }

        public static void PostRender()
        {
            Glfw.SwapBuffers(window);
        }

        public static void DestroyWindow()
        {
            Glfw.Terminate();
        }
    }
}
