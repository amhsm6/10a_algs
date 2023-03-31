import matplotlib.pyplot as plt
import numpy as np

algs = ["alg1", "alg2"]
time = [1, 3]

x_pos = np.arange(len(algs))

plt.bar(algs, time, align = "center", alpha = 0.5)
plt.xticks(x_pos, algs)
plt.ylabel("time")
plt.show()
