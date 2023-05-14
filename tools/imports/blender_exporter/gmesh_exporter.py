import bpy
from array import array
from numpy import concatenate
from math import sqrt

def compute_normals(vertices):
    v1, v2, v3 = vertices
    p = [v2[0] - v1[0], v2[1] - v1[1], v2[2] - v1[2]]
    q = [v3[0] - v1[0], v3[1] - v1[1], v3[2] - v1[2]]
    n = [p[1] * q[2] - p[2] * q[1], p[2] * q[0] - p[0] * q[2], p[0] * q[1] - p[1] * q[0]]
    n_mag = sqrt(n[0] * n[0] + n[1] * n[1] + n[2] * n[2])
    return [n[0] / n_mag, n[1] / n_mag, n[2] / n_mag]

def apply_transform(vertex):
    return [vertex[0], vertex[2], vertex[1], vertex[3], vertex[5], vertex[4], vertex[6], vertex[7]]

# https://en.wikipedia.org/wiki/Batch_file
def write_some_data(context, filepath):
    print("running write_some_data...")
    scene = context.scene
    viewlayer = context.view_layer
    obs = context.selected_objects
    for ob in obs:
        viewlayer.objects.active = ob
        ob.select_set(True)
        # create file for this object
        mesh = ob.data
        f = open(filepath, 'wb')
        # write headers 
        vertex_length = len(mesh.vertices)
        triangles_length = sum([1 for face in mesh.polygons if len(face.vertices) == 3])
        # compute vertices
        smooth_shading = False
        if smooth_shading:
            header = array('L', [vertex_length, triangles_length]) # L is unsigned long, so u32
            vertices = [[v.co[0], v.co[1], v.co[2], v.normal[0], v.normal[1], v.normal[2], 0, 0] for v in mesh.vertices]
            # set the uv for the vertices
            uv_layer = mesh.uv_layers.active.data
            for loop in mesh.loops:
                vertices[loop.vertex_index][6] = uv_layer[loop.index].uv[0]
                vertices[loop.vertex_index][7] = uv_layer[loop.index].uv[1]
            # compute triangles
            triangles_data = array('L', concatenate([[mesh.loops[i].vertex_index for i in f.loop_indices][::-1] for f in mesh.polygons if len(f.vertices) == 3]))
        else:
            header = array('L', [triangles_length * 3, triangles_length]) # L is unsigned long, so u32
            # temp array of vertices to compute final vertices data, will be rearranged for flat shading
            temp_vertices = [[v.co[0], v.co[1], v.co[2], v.normal[0], v.normal[1], v.normal[2], 0, 0] for v in mesh.vertices]
            # set the uv for the vertices
            uv_layer = mesh.uv_layers.active.data
            for loop in mesh.loops:
                temp_vertices[loop.vertex_index][6] = uv_layer[loop.index].uv[0]
                temp_vertices[loop.vertex_index][7] = uv_layer[loop.index].uv[1]
            # compute triangles
            temp_triangles = array('L', concatenate([[mesh.loops[i].vertex_index for i in f.loop_indices] for f in mesh.polygons if len(f.vertices) == 3]))
            vertices = [temp_vertices[i].copy() for i in temp_triangles]
            # compute normals for each face
            for i in range(0, len(vertices), 3):
                n = compute_normals(vertices[i:i+3])
                for j in range(i, i+3):
                    vertices[j][3:6] = n[::]
            triangles_data = array('L', [i for i in range(len(vertices))])
        for i in range(len(vertices)):
            vertices[i] = apply_transform(vertices[i])
        vertex_data = array('f', concatenate(vertices))
        # write data to file
        # write triangles data
        header.tofile(f)
        vertex_data.tofile(f)
        triangles_data.tofile(f)
        ob.select_set(False)
        f.close()
    return {'FINISHED'}

# ExportHelper is a helper class, defines filename and
# invoke() function which calls the file selector.
from bpy_extras.io_utils import ExportHelper
from bpy.types import Operator

class ExportSomeData(Operator, ExportHelper):
    """This appears in the tooltip of the operator and in the generated docs"""
    bl_idname = "gear_export.mesh_export"
    bl_label = "Export to Gear Mesh"

    # ExportHelper mixin class uses this
    filename_ext = ".gmesh"
    filter_glob: bpy.props.StringProperty(default="*.gmesh", options={'HIDDEN'}, maxlen=255)

    def execute(self, context):
        return write_some_data(context, self.filepath)

def register():
    bpy.utils.register_class(ExportSomeData)
def unregister():
    bpy.utils.unregister_class(ExportSomeData)

if __name__ == "__main__":
    register()
    bpy.ops.gear_export.mesh_export('INVOKE_DEFAULT')