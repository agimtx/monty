import aq
from aq import hello

# === Basic aq module import ===
assert aq.hello() == 'hello', 'aq.hello returns greeting'

# === from-import binding ===
assert hello() == 'hello', 'from aq import hello binds function'
