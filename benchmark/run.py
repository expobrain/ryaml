from datetime import datetime
import gc
import os

import yaml
import ryaml

# Be sure that PyYAML is usign the libyaml extension
assert(yaml.__with_libyaml__ is True)

# Disable GC to not influence the benchmark
gc.disable()

print('Loading test data...')
data = open(os.path.join(os.path.dirname(__file__), 'data.yaml')).read()

print('Starting benchmark...')
for mod in (yaml, ryaml):
    # Decode speed
    start = datetime.now()
    decoded = mod.safe_load(data)
    end = datetime.now()
    print("{} decode time: {}".format(mod.__name__, end - start))

    # Encode speed
    start = datetime.now()
    mod.safe_dump(decoded)
    end = datetime.now()

    print("{} encode time: {}".format(mod.__name__, end - start))
