
namespace Sc.Native
{
    public static class Version
    {
        #if UNITY_EDITOR
        public const string version = "sc_native_1677592955";
        #else
        public static string version = "sc_native";
        #endif
    }
}