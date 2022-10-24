
using System.Numerics;
using GLFW;
using OpenGL.Core;

namespace Delphi
{
    static class DisplayManager
    {
        public static Window Window;
        public static Vector2 WindowSize;

        public static void CreateWindow(int width = 800, int height = 600, string title = "New Window", int vsync = 0)
        {
            WindowSize = new Vector2(width, height);

            // GLFW initialization
            Glfw.Init();
            Glfw.WindowHint(Hint.ContextVersionMajor, 4);
            Glfw.WindowHint(Hint.ContextVersionMinor, 3);
            Glfw.WindowHint(Hint.OpenglProfile, Profile.Core);

            Glfw.WindowHint(Hint.Focused, true);
            Glfw.WindowHint(Hint.Resizable, false);

            // create new window
            Window = Glfw.CreateWindow(width, height, title, GLFW.Monitor.None, Window.None);
            if (Window == Window.None)
            {
                // window creation failed
                return;
            }

            Glfw.MakeContextCurrent(Window);
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
            Glfw.SwapBuffers(Window);
        }

        public static void DestroyWindow()
        {
            Glfw.Terminate();
        }
    }
}
