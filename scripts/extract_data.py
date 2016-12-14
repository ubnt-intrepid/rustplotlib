
def extract_data():
    fname = __file__
    if fname.endswith('.pyc'):
        fname = fname[0:-1]

    tgbegin = '\n#==>\n#'.encode()
    tgend = '\n#<==\n'.encode()

    data = open(fname, 'rb').read()
    data = data[data.find(tgbegin) + len(tgbegin) : data.find(tgend)]
    data = base64.b64decode(data)

    return msgpack.unpackb(data)

fig = plt.figure()
make_figure(fig, extract_data())

if __name__ == '__main__':
    fig.savefig('result.png')
