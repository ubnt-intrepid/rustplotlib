#!/usr/bin/env python

import base64
import msgpack
import sys
import matplotlib.pyplot as plt

tgbegin = '\n#==>\n#'.encode()
tgend   = '\n#<==\n'.encode()

def plot_scatter(ax, data):
    x1, x2 = data[0:2]
    l, c, m = map(lambda s: s.decode('utf-8'), data[2:5])
    ax.scatter(x1, x2, label=l, color=c, marker=m)

def make_plot(ax, data):
    plot_type, data = data
    if plot_type == 0: # scatter
        data = data[0]
        plot_scatter(ax, data)

def make_axes(ax, data):
    plot   = data[0]
    xlabel = data[1].decode('utf-8')
    ylabel = data[2].decode('utf-8')
    grid   = data[3]
    for p in plot:
        make_plot(ax, p)
    ax.set_xlabel(xlabel)
    ax.set_ylabel(ylabel)
    ax.grid(grid)
    ax.legend(loc='upper left')

def make_figure(fig, data):
    # TODO: support for multiple subplots
    data = data[0]
    ax = fig.add_subplot(1, 1, 1)
    make_axes(ax, data)

def read_data():
    try:
        fname = __file__
        if fname.endswith('.pyc'):
            fname = fname[0:-1]
        data = open(fname, 'rb').read()
        data = data[data.find(tgbegin) + len(tgbegin) : data.find(tgend)]
    except NameError:
        data = sys.stdin.buffer.read()

    return msgpack.unpackb(base64.b64decode(data))

def evaluate(data):
    data = msgpack.unpackb(base64.b64decode(data))
    fig = plt.figure()
    make_figure(fig, data)
    return fig
