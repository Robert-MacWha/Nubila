using GLFW;
using System.Numerics;

namespace Nubila
{
    internal class Input
    {
        private static Dictionary<Keys, bool> keyStatus = new Dictionary<Keys, bool>();
        private static Vector2 prevMousePos;

        // returns true while a key is pressed
        public static bool GetKey(Keys key)
        {
            bool status = keyStatus[key] && (Glfw.GetKey(DisplayManager.window, key) == InputState.Press);
            return status;
        }

        // returns true the frame a key is pressed
        public static bool GetKeyDown(Keys key)
        {
            bool status = !keyStatus[key] && (Glfw.GetKey(DisplayManager.window, key) == InputState.Press);
            return status;
        }

        // returns true the frame a key is released
        public static bool GetKeyUp(Keys key)
        {
            bool status = keyStatus[key] && !(Glfw.GetKey(DisplayManager.window, key) == InputState.Press);
            return status;
        }

        // return the raw mouse position
        public static Vector2 GetMousePosition()
        {
            double x;
            double y;
            Glfw.GetCursorPosition(DisplayManager.window, out x, out y);

            return new Vector2((float)x, (float)y);
        }

        // returns the mouse's movement in the past frame
        public static Vector2 GetMouseDelta()
        {
            return GetMousePosition() - prevMousePos;
        }

        // INTERNAL: initialize the key status dict
        public static void Initialize()
        {
            // initialize keys
            foreach (Keys key in Enum.GetValues(typeof(Keys)))
            {
                keyStatus.Add(key, false);
            }

            // initialize mouse
            // Glfw.SetInputMode(DisplayManager.window, InputMode.Cursor, (int)CursorMode.Disabled);
            prevMousePos = GetMousePosition();
        }

        // INTERNAL: update all input status
        public static void UpdateInputs()
        {
            // update keys
            foreach(Keys key in Enum.GetValues(typeof(Keys)))
            {
                if (key != Keys.Unknown) // passing unknown breaks GetKey()
                {
                    keyStatus[key] = Glfw.GetKey(DisplayManager.window, key) == InputState.Press;
                }
            }

            // update mouse
            prevMousePos = GetMousePosition();
        }
    }
}
