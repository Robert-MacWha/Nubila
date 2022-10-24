using GLFW;

namespace Delphi
{
    abstract class Engine
    {
        // game engine loop
        public void Run()
        {
            Initialize();
            DisplayManager.CreateWindow(800, 600, "Voxel Engine", 0);
            LoadContent();

            while (!Glfw.WindowShouldClose(DisplayManager.Window))
            {
                Time.DeltaTime = (float)Glfw.Time - Time.ElapsedTime;
                Time.ElapsedTime = (float)Glfw.Time;

                Update();
                Glfw.PollEvents();

                DisplayManager.PreRender();
                Render();
                DisplayManager.PostRender();
            }

            DisplayManager.DestroyWindow();
        }

        protected abstract void Initialize();
        protected abstract void LoadContent();

        protected abstract void Update();
        protected abstract void Render();
    }
}
