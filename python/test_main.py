from numpy.core.multiarray import dtype
from main import *

import numpy as np

def get_list(size):
    arr = np.array(list(range(0, size)), dtype=np.int32)
    # arr = np.arange(0, size, dtype=np.int32)
    np.random.shuffle(arr)
    arr = np.asarray(list(arr))
    arr = arr[:-1]

    return arr

def setup():
    return (), {'arr': get_list(100_000)}

def test_just_np_sort_1M(benchmark):

    def setup():
        # can optionally return a (args, kwargs) tuple
        return (), {'arr': np.array(get_list(1_000_000))}

    benchmark.pedantic(just_np_sort, setup=setup, warmup_rounds=5, rounds=100)

def test_just_np_sort_5M(benchmark):

    def setup():
        # can optionally return a (args, kwargs) tuple
        return (), {'arr': np.array(get_list(5_000_000))}

    benchmark.pedantic(just_np_sort, setup=setup, warmup_rounds=5, rounds=100)

def test_find_smallet_int_100k(benchmark):

    def setup():
        # can optionally return a (args, kwargs) tuple
        return (), {'arr': np.array(get_list(100_000))}

    benchmark.pedantic(find_smallet_int, setup=setup, warmup_rounds=20, rounds=100)

def test_find_smallet_int_1M(benchmark):

    def setup():
        # can optionally return a (args, kwargs) tuple
        return (), {'arr': np.array(get_list(1_000_000))}

    benchmark.pedantic(find_smallet_int, setup=setup, warmup_rounds=20, rounds=100)

def test_find_smallet_int_2M(benchmark):

    def setup():
        # can optionally return a (args, kwargs) tuple
        return (), {'arr': np.array(get_list(2_000_000))}

    benchmark.pedantic(find_smallet_int, setup=setup, warmup_rounds=20, rounds=100)

def test_find_smallet_int_5M(benchmark):

    def setup():
        # can optionally return a (args, kwargs) tuple
        return (), {'arr': np.array(get_list(5_000_000))}

    benchmark.pedantic(find_smallet_int, setup=setup, warmup_rounds=20, rounds=100)
