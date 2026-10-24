"""Tests of gl_table.py (design D2 of m4-sub-gl): the old compiler's rules for reading gl.h.

Run: python -m unittest discover -s tools/opengl -p "test_*.py"
"""

import pathlib
import sys
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import gl_table


def constants(text):
    return gl_table.scan(text)[0]


def routine(line):
    r = gl_table.read_routine(line)
    assert r is not None, line
    return r


class DefineTest(unittest.TestCase):
    def test_hex_and_decimal(self):
        got = constants("#define GL_A 0x00004000\r\n#define GL_B 12\r\n")
        self.assertEqual(got, [("_GL_A", 16384), ("_GL_B", 12)])

    def test_all_ones_stay_32_bits(self):
        self.assertEqual(constants("#define GL_ALL 0xffffffff\n"), [("_GL_ALL", 4294967295)])

    def test_a_define_naming_an_earlier_one(self):
        got = constants("#define GL_DOUBLE 0x140A\n#define GL_DOUBLE_EXT GL_DOUBLE\n")
        self.assertEqual(got, [("_GL_DOUBLE", 5130), ("_GL_DOUBLE_EXT", 5130)])

    def test_a_define_naming_a_later_or_unknown_one_is_dropped(self):
        got = constants("#define GL_X GL_Y\n#define GL_Y 1\n#define GL_Z GL_NOPE\n")
        self.assertEqual(got, [("_GL_Y", 1)])

    def test_lower_case_names_and_empty_values_are_dropped(self):
        got = constants("#define __gl_h_\n#define __GL_H__\n#define GL_EXT_bgra 1\n#define GL_OK 2\n")
        self.assertEqual(got, [("_GL_OK", 2)])

    def test_a_name_with_its_own_underscore_keeps_it(self):
        self.assertEqual(constants("#define _GL_X 3\n"), [("_GL_X", 3)])

    def test_a_value_it_does_not_read_is_named(self):
        with self.assertRaises(gl_table.GlError):
            constants("#define GL_X 1u\n")


class RoutineTest(unittest.TestCase):
    def test_a_sub_with_arguments_has_a_helper(self):
        r = routine("WINGDIAPI void APIENTRY glViewport(GLint x,GLint y,GLsizei width,GLsizei height);")
        self.assertEqual((r.name, r.c_name, r.ret, r.args), ("_glViewport", "call_glViewport", None, ["I32"] * 4))
        self.assertEqual(
            r.helper,
            "void call_glViewport(int32 a,int32 b,int32 c,int32 d){\nif (!sub_gl_called) error(270);\n"
            "glViewport((GLint)a,(GLint)b,(GLsizei)c,(GLsizei)d);\n}\n",
        )

    def test_no_arguments_no_helper(self):
        r = routine("WINGDIAPI void APIENTRY glFlush(void);")
        self.assertEqual((r.c_name, r.args, r.helper), ("glFlush", [], None))
        r = routine("WINGDIAPI GLenum APIENTRY glGetError(void);")
        self.assertEqual((r.c_name, r.ret, r.helper), ("glGetError", "U32", None))

    def test_a_pointer_is_an_offset(self):
        r = routine("WINGDIAPI void APIENTRY glGetIntegerv(GLenum pname,GLint *params);")
        self.assertEqual(r.args, ["U32", "Off"])
        self.assertIn("void call_glGetIntegerv(uint32 a,ptrszint b){", r.helper)
        self.assertIn("glGetIntegerv((GLenum)a,(GLint*)b);", r.helper)

    def test_a_pointer_to_a_pointer_is_an_offset(self):
        r = routine("WINGDIAPI void APIENTRY glGetPointerv(GLenum pname,GLvoid **params);")
        self.assertEqual(r.args, ["U32", "Off"])
        self.assertIn("glGetPointerv((GLenum)a,(GLvoid**)b);", r.helper)

    def test_a_const_pointer_argument(self):
        r = routine("WINGDIAPI void APIENTRY glColor3fv(const GLfloat *v);")
        self.assertEqual(r.args, ["Off"])
        self.assertIn("glColor3fv((GLfloat*)a);", r.helper)

    def test_a_const_pointer_result_is_glgetstrings_string(self):
        r = routine("WINGDIAPI const GLubyte *APIENTRY glGetString(GLenum name);")
        self.assertEqual((r.c_name, r.ret, r.args), ("call_glGetString", "Str", ["U32"]))
        self.assertIn("ptrszint call_glGetString(uint32 a){", r.helper)
        self.assertIn("return (ptrszint)(GLubyte*)glGetString((GLenum)a);", r.helper)

    def test_a_function_result(self):
        r = routine("WINGDIAPI GLboolean APIENTRY glIsEnabled(GLenum cap);")
        self.assertEqual((r.ret, r.args), ("U8", ["U32"]))
        self.assertIn("return (uint8)(GLboolean)glIsEnabled((GLenum)a);", r.helper)

    def test_a_blank_before_the_parenthesis(self):
        r = routine("WINGDIAPI void APIENTRY glDepthRange (GLclampd zNear,GLclampd zFar);")
        self.assertEqual((r.name, r.args), ("_glDepthRange", ["F64", "F64"]))

    def test_other_lines_are_not_routines(self):
        self.assertIsNone(gl_table.read_routine("typedef void (APIENTRY *PFNGLX)(GLenum pname);"))
        self.assertIsNone(gl_table.read_routine("WINGDIAPI void glNoEntry(GLenum a);"))

    def test_an_unknown_type_is_named(self):
        with self.assertRaises(gl_table.GlError):
            routine("WINGDIAPI void APIENTRY glX(GLhalf a);")

    def test_glu_perspective_comes_last(self):
        _, routines = gl_table.scan("WINGDIAPI void APIENTRY glFlush(void);\n")
        self.assertEqual([r.name for r in routines], ["_glFlush", "_gluPerspective"])
        self.assertEqual(routines[-1].args, ["F64"] * 4)


class OutputTest(unittest.TestCase):
    def test_callsite_programs(self):
        sub = routine("WINGDIAPI void APIENTRY glClearColor(GLclampf r,GLclampf g,GLclampf b,GLclampf a);")
        text = gl_table.callsite_program(sub)
        self.assertIn("DIM a AS SINGLE, b AS SINGLE, c AS SINGLE, d AS SINGLE\n_glClearColor a, b, c, d\nEND\n", text)
        self.assertIn("\nSUB _GL\nEND SUB\n", text)
        func = routine("WINGDIAPI GLenum APIENTRY glGetError(void);")
        self.assertIn("DIM res AS _UNSIGNED LONG\nres = _glGetError\nEND\n", gl_table.callsite_program(func))

    def test_two_names_differing_by_case_are_named(self):
        with self.assertRaises(gl_table.GlError):
            gl_table.unique(["_GL_A", "_gl_a"], "constants")

    def test_the_files_are_what_it_writes(self):
        self.assertEqual(gl_table.main(["--check"]), 0)


if __name__ == "__main__":
    unittest.main()
