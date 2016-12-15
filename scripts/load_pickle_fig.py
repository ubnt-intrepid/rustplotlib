#!/usr/bin/env python

import matplotlib as mpl
mpl.use('qt5agg')

import matplotlib.pyplot as plt
import numpy as np
import pickle as pl

fig = pl.load(open('scatter_native.png.pkl', 'rb'))
plt.show()