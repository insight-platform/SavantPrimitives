import numpy as np
from savant_rs.utils import *
from timeit import default_timer as timer

num = 10_000

t = timer()
v = np.zeros((128, 4), dtype='float')
m = None
for _ in range(num):
    m = ndarray_to_matrix(v)

print(f"NP64>NALGEBRA {num} Time:", timer() - t)

t = timer()
for _ in range(num):
    v = matrix_to_ndarray(m)

print(f"NALGEBRA>NP64 {num} Time:", timer() - t)


t = timer()
v = np.zeros((128, 4), dtype='float32')
m = None
for _ in range(num):
    m = ndarray_to_matrix(v)
print(f"NP32>NALGEBRA {num} Time:", timer() - t)

t = timer()
for _ in range(num):
    v = matrix_to_ndarray(m)

print("NALGEBRA>NP32 {num} Time:", timer() - t)
