import timeit
from test_main import get_list
from main import just_np_sort

setup = """
from test_main import get_list
from main import just_np_sort
l = get_list(5_000_000)
"""

run = """
just_np_sort(l)
"""
print(get_list(10))
print(just_np_sort(get_list(10)))

runs = 300
print(timeit.timeit(run, setup=setup, number=runs) / runs * 1000, 'ms')
