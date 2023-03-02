using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using UnityEngine;

public class Intersection : MonoBehaviour
{
    public MeshFilter a;
    public MeshFilter b;
    void Update()
    {
        var line = Sc.Native.Triangles.Triangle.Intersect(new Sc.Native.Triangles.Triangle(a), new Sc.Native.Triangles.Triangle(b));
        Debug.DrawLine(line.start, line.end, Color.red);
    }
}

namespace Sc.Native
{
    public struct Line
    {
        public Vector3 start;
        public Vector3 end;
    }

    public static class Triangles
    {
        
        public struct Triangle
        {
            public Vector3 a;
            public Vector3 b;
            public Vector3 c;

            public Triangle(Vector3 a, Vector3 b, Vector3 c)
            {
                this.a = a;
                this.b = b;
                this.c = c;
            }

            public Triangle(MeshFilter filter) {
                var mesh = filter.mesh;
                var vertices = mesh.vertices;
                var triangles = mesh.triangles;

                a = filter.transform.TransformPoint(vertices[triangles[0]]);
                b = filter.transform.TransformPoint(vertices[triangles[1]]);
                c = filter.transform.TransformPoint(vertices[triangles[2]]);
            }

            [DllImport(Sc.Native.Version.version, EntryPoint = "intersect")]
            public extern static Line Intersect(Triangle a, Triangle b);
        }
    }
}