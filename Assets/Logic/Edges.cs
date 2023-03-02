using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using UnityEngine;

namespace Sc.Native
{
    public class Edges : MonoBehaviour
    {
        public Transform light;
        
        static MeshFilter[] casters;

        private void Awake()
        {
            if(Edges.casters == null) {
                var casters = Resources.FindObjectsOfTypeAll<ShadowCaster>();
                Edges.casters = new MeshFilter[casters.Length];
                for (int i = 0; i < casters.Length; i++)
                {
                    Edges.casters[i] = casters[i].GetComponent<MeshFilter>();
                }
            }

            var array = new int[] { 1, 2, 3, 4, 5 };

            unsafe
            {
                fixed (int* ptr = array)
                {
                    var len = test_sum((IntPtr)ptr, array.Length);
                    Debug.Log("Sum: " + len);
                }
            }

            var array2 = new Vector3[] { Vector3.one, Vector3.one, Vector3.one, Vector3.one, Vector3.one };

            unsafe
            {
                fixed (Vector3* ptr = array2)
                {
                    var len = vec_sum((IntPtr)ptr, array2.Length);
                    Debug.Log("Sum: " + len);
                }
            }

            generate_dynamic_array(out IntPtr arrayPtr, out int arrayLen);
            var array3 = new int[arrayLen];
            Marshal.Copy(arrayPtr, array3, 0, arrayLen);
            Debug.Log("Array: " + array3.Length);
        }

        private void CalculateEdges(MeshFilter filter) {
            (T[], int, Action) InteropArray<T>(T[] array) {
                IntPtr ptr = Marshal.AllocHGlobal(array.Length * Marshal.SizeOf(typeof(T)));
                var bytesArray = new byte[array.Length * Marshal.SizeOf(typeof(T))];
                Buffer.BlockCopy(array, 0, bytesArray, 0, bytesArray.Length);
                Marshal.Copy(bytesArray, 0, ptr, array.Length);
                return (array, ptr.ToInt32(), () => Marshal.FreeHGlobal(ptr));
                
            }
            
            unsafe
            {
                // var mesh = filter.sharedMesh;
                // var vertices = mesh.vertices;
                // var positions = new float[vertices.Length * 3];
                // for (int i = 0; i < vertices.Length; i++)
                // {
                //     positions[i * 3] = vertices[i].x;
                //     positions[i * 3 + 1] = vertices[i].y;
                //     positions[i * 3 + 2] = vertices[i].z;
                // }
                // var triangles = mesh.triangles;
                // var transform = filter.transform.localToWorldMatrix;

                // var (verticesArray, verticesPtr, freeVertices) = InteropArray(positions);
                // var (trianglesArray, trianglesPtr, freeTriangles) = InteropArray(triangles);
                // var transformPtr = &transform.m00;

                // var edgesPtr = CalculateEdgesNative((float*)verticesPtr, vertices.Length, (int*)trianglesPtr, triangles.Length, transformPtr, out int len);
                // var edgesPtr = CalculateEdgesNative(null, 0, null, 0, null, out int len);
                // var edges = new float[len];
                // Marshal.Copy(edgesPtr, edges, 0, len);

                // Debug.Log("Edges: " + len);

                // freeVertices();
                // freeTriangles();
            }
        }

        struct Caster {
            public Vector3[] vertices;
            public int[] triangles;
            public Matrix4x4 edges;
        }

        // [DllImport(Version.version, EntryPoint = "calculate_edges")]
        // private unsafe static extern IntPtr CalculateEdgesNative(float* vertices, int verticesLength, int* triangles, int trianglesLength, float* transform, out int len);

        [DllImport(Version.version)]
        private unsafe static extern int test_sum(IntPtr array, int len);

        [DllImport(Version.version)]
        private unsafe static extern Vector3 vec_sum(IntPtr array, int len);

        [DllImport(Version.version)]
        public static extern void generate_dynamic_array(out IntPtr arrayPtr, out int arrayLen);
    }
}
