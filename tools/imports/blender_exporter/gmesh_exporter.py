import bpy
from array import array
from numpy import concatenate

# https://en.wikipedia.org/wiki/Batch_file
def write_some_data(context, filepath):
    print("running write_some_data...")
    scene = context.scene
    viewlayer = context.view_layer
    obs = [o for o in scene.objects if o.type == 'MESH']
    bpy.ops.object.select_all(action='DESELECT')    
    for ob in obs:
        viewlayer.objects.active = ob
        ob.select_set(True)
        # create file for this object
        mesh = ob.data
        f = open(filepath, 'wb')
        # write headers 
        vertex_length = len(mesh.vertices)
        triangles_length = sum([1 for face in mesh.polygons if len(face.vertices) == 3])
        header = array('L', [vertex_length, triangles_length]) # L is unsigned long, so u32
        header.tofile(f)
        # write vertex data (f is float32 type)
        vertex_data = array('f', concatenate([[v.co[0], v.co[1], v.co[2], v.normal[0], v.normal[1], v.normal[2], 0, 0] for v in mesh.vertices]))
        # todo : uvs
        vertex_data.tofile(f)
        # write triangles data
        triangles_data = array('L', concatenate([[mesh.loops[i].vertex_index for i in f.loop_indices] for f in mesh.polygons if len(f.vertices) == 3]))
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