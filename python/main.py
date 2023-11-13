import numpy as np

def find_smallet_int(arr):
    sorted_arr = np.sort(arr)
    return sorted_arr[np.where(np.diff(sorted_arr) > 1)][0]+1

def subtract(arr, arr2):
    return arr - arr2

def find_smallest_int_ram(arr):
    ram = np.zeros(2_000_001, dtype=np.bool_)
    ram[arr] = np.bool_(1)
    return np.argmin(ram)

def just_sort(arr):
    return sorted(arr)

def just_np_sort(arr):
    return np.sort(arr, kind="quicksort")

if __name__ == '__main__':
    print(find_smallet_int([1,2,3,4,6]))
