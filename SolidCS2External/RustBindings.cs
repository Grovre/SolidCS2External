using System.Numerics;
using System.Runtime.InteropServices;

namespace SolidCS2External;

public static class RustBindings
{
    public const string DllName = "cs2_rs.dll";

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern Vector3 aim_at(
        Vector3 origin,
        Vector3 viewOffset,
        Vector3 viewAngles,
        Vector3 target,
        float smoothingFactor);
}