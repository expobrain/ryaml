import ryaml
import unittest


class SafeLoadTests(unittest.TestCase):

    def test_load_simple(self):
        yaml = "result: 42"
        expected = {'result': 42}

        result = ryaml.safe_load(yaml)

        assert result == expected

    def test_load_key_is_number(self):
        yaml = "42: is a number"
        expected = {42: 'is a number'}

        result = ryaml.safe_load(yaml)

        assert result == expected

    def test_load_value_is_array(self):
        yaml = (
            'array:\n'
            '- 1\n'
            '- 2\n'
            '- 3'
        )
        expected = {'array': [1, 2, 3]}

        result = ryaml.safe_load(yaml)

        assert result == expected

    def test_load_nested_dict(self):
        yaml = (
            'result:\n'
            '  number: 42'
        )
        expected = {'result': {'number': 42}}

        result = ryaml.safe_load(yaml)

        assert result == expected

    def test_load_null(self):
        yaml = 'result:'
        expected = {'result': None}

        result = ryaml.safe_load(yaml)

        assert result == expected

    def test_load_float(self):
        yaml = 'result: 1.2'
        expected = {'result': 1.2}

        result = ryaml.safe_load(yaml)

        assert result == expected

    def test_load_boolean(self):
        yaml = 'result: true'
        expected = {'result': True}

        result = ryaml.safe_load(yaml)

        assert result == expected


if __name__ == '__main__':
    unittest.main()
