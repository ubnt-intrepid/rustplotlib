#!/usr/bin/env python

import base64
import msgpack
import sys
import matplotlib.pyplot as plt


def str_decode(s):
    if s is None:
        return None
    return s.decode('utf-8')


class Scatter(object):
    def __init__(self, data):
        self.x1 = data[0]
        self.x2 = data[1]
        self.config = ScatterConfig(data[2])

    def apply(self, ax):
        ax.scatter(self.x1, self.x2, **self.config.as_dict())

class ScatterConfig(object):
    def __init__(self, data):
        self.label  = str_decode(data[0])
        self.color  = str_decode(data[1])
        self.marker = str_decode(data[2])

    def as_dict(self):
        return dict(
            label=self.label,
            color=self.color,
            marker=self.marker,
        )


class Line2D(object):
    def __init__(self, data):
        self.x1 = data[0]
        self.x2 = data[1]
        self.config = Line2DConfig(data[2])

    def apply(self, ax):
        ax.plot(self.x1, self.x2, **self.config.as_dict())

class Line2DConfig(object):
    def __init__(self, data):
        self.label      = str_decode(data[0])
        self.color      = str_decode(data[1])
        self.marker     = str_decode(data[2])
        self.linestyle  = str_decode(data[3])
        self.linewidth  = data[4]

    def as_dict(self):
        return dict(
            label=self.label,
            color=self.color,
            marker=self.marker,
            linestyle=self.linestyle,
            linewidth=self.linewidth,
        )


def plot_data(data):
    if data[0] == 0: # scatter
        return Scatter(data[1][0])
    elif data[0] == 1: # plot
        return Line2D(data[1][0])
    else:
        return None


class Axes2D(object):
    def __init__(self, data):
        self.plot_data = list(map(plot_data, data[0]))
        self.config = Axes2DConfig(data[1])

    def apply(self, ax):
        for p in self.plot_data:
            p.apply(ax)
        self.config.apply(ax)

class Axes2DConfig(object):
    def __init__(self, data):
        self.xlabel = str_decode(data[0])
        self.ylabel = str_decode(data[1])
        self.grid   = data[2]
        self.legend = str_decode(data[3])
        self.xlim   = data[4]
        self.ylim   = data[5]

    def apply(self, ax):
        if self.xlabel:
            ax.set_xlabel(self.xlabel)
        if self.ylabel:
            ax.set_ylabel(self.ylabel)
        ax.grid(self.grid)
        if self.legend:
            ax.legend(loc=self.legend)
        if self.xlim:
            ax.set_xlim(self.xlim)
        if self.ylim:
            ax.set_ylim(self.ylim)


class Figure(object):
    def __init__(self, data):
        # TODO: support for multiple subplots
        self.axes = Axes2D(data[0])

    def apply(self, fig):
        ax = fig.add_subplot(1, 1, 1)
        self.axes.apply(ax)


def unpack(data=None):
    """
    Returns unpacked data from string.
    If `data` is None, it will collect packed data from the footer of script, or standard input.
    """
    tgbegin = '\n#==>\n#'.encode()
    tgend   = '\n#<==\n'.encode()
    if not data:
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
    """
    Create a figure object and apply all operations.

    Arguments:
      data:
        Packed data which contains all of operations applied to `matplotlib.figure.Figure`.
        If `data` is a `str`, it will be unpacked by using `unpack()`.
    
    Returns:
      The instance of `matplotlib.figure.Figure` which applied all operations.
    """
    if data is None:
        data = unpack()
    elif isinstance(data, str):
        data = unpack(data)
    fig = plt.figure()
    Figure(data).apply(fig)
    return fig

def main():
    fig = evaluate(None)
    fig.savefig('result.png')
