# -*- coding: UTF-8 -*-

from __future__ import unicode_literals

import ryaml
import unittest


class SafeDumpTests(unittest.TestCase):

    def test_simple(self):
        data = {'result': 42}
        expected = '"result": 42'

        result = ryaml.safe_dump(data)

        assert result == expected

    def test_key_is_integer(self):
        data = {42: 'is a number'}
        expected = '42: "is a number"'

        result = ryaml.safe_dump(data)

        assert result == expected

    def test_key_is_list(self):
        data = {'array': [1, 2, 3]}
        expected = (
            '"array": \n'
            '  - 1\n'
            '  - 2\n'
            '  - 3'
        )

        result = ryaml.safe_dump(data)

        assert result == expected

    def test_nested_dict(self):
        expected = (
            '"result": \n'
            '  "number": 42'
        )
        data = {'result': {'number': 42}}

        result = ryaml.safe_dump(data)

        assert result == expected

    def test_value_is_null(self):
        data = {'result': None}
        expected = '"result": ~'

        result = ryaml.safe_dump(data)

        assert result == expected

    def test_value_is_float(self):
        data = {'result': 1.2}
        expected = '"result": 1.2'

        result = ryaml.safe_dump(data)

        assert result == expected

    def test_value_is_boolean(self):
        data = {'result': True}
        expected = '"result": true'

        result = ryaml.safe_dump(data)

        assert result == expected

    def test_value_is_unicode(self):
        data = {'result': '€'}
        expected = '"result": "€"'

        result = ryaml.safe_dump(data)

        assert result == expected
