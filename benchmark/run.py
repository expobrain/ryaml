from datetime import datetime
import gc
import os

import yaml
import ryaml

# Be sure that PyYAML is usign the libyaml extension
assert(yaml.__with_libyaml__ is True)

from yaml import CSafeLoader

print('Loading test data...')
data = open(os.path.join(os.path.dirname(__file__), 'data.yaml')).read()

print('Starting benchmark...')

# YAML
# Decode speed
start = datetime.now()
decoded = yaml.load(data, Loader=CSafeLoader)
end = datetime.now()
print("{} decode time: {}".format(yaml.__name__, end - start))

# Encode speed
start = datetime.now()
yaml.safe_dump(decoded)
end = datetime.now()

print("{} encode time: {}".format(yaml.__name__, end - start))

# rYAML
# Decode speed
start = datetime.now()
decoded = ryaml.safe_load(data)
end = datetime.now()
print("{} decode time: {}".format(ryaml.__name__, end - start))

# Encode speed
start = datetime.now()
ryaml.safe_dump(decoded)
end = datetime.now()

print("{} encode time: {}".format(ryaml.__name__, end - start))
