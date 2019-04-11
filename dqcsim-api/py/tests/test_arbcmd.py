import unittest

from dqcsim.common import ArbCmd, ArbData
from dqcsim import raw

class Tests(unittest.TestCase):

    def test_constructor(self):
        self.assertEqual(repr(ArbCmd("a", "b")), "ArbCmd('a', 'b')")

        with self.assertRaises(TypeError):
            ArbCmd()

        with self.assertRaises(ValueError):
            ArbCmd("$#^", "dkbng")

        with self.assertRaises(ValueError):
            ArbCmd(" hello ", "dkbng")

        self.assertEqual(repr(ArbCmd("a", "b", b"c", d="e")), "ArbCmd('a', 'b', b'c', d='e')")

    def test_getters(self):
        c = ArbCmd("a", "b")
        self.assertEqual(c.iface(), "a")
        self.assertEqual(c.oper(), "b")

    def test_eq(self):
        a = ArbCmd("a", "b")
        self.assertTrue(a == ArbCmd("a", "b"))
        self.assertFalse(a != ArbCmd("a", "b"))
        self.assertFalse(a == ArbCmd("a", "x"))
        self.assertFalse(a == ArbCmd("a", "x"))
        self.assertFalse(a == ArbData())
        self.assertFalse(a == ArbCmd("a", "x", b"a"))

    def test_handles(self):
        a = ArbCmd('x', 'y', b'a', b'b', b'c', b=3, c=4, d=5)
        a_handle = a.to_raw()
        self.maxDiff = None
        self.assertEqual(raw.dqcs_handle_dump(a_handle), """ArbCmd(
    ArbCmd {
        interface_identifier: "x",
        operation_identifier: "y",
        data: ArbData {
            json: Object(
                {
                    String(
                        "b"
                    ): U64(
                        3
                    ),
                    String(
                        "c"
                    ): U64(
                        4
                    ),
                    String(
                        "d"
                    ): U64(
                        5
                    )
                }
            ),
            args: [
                [
                    97
                ],
                [
                    98
                ],
                [
                    99
                ]
            ]
        }
    }
)""")

        self.assertEqual(ArbCmd.from_raw(a_handle), a)
        raw.dqcs_handle_delete(a_handle)

if __name__ == '__main__':
    unittest.main()
