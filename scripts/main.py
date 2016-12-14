
if __name__ == '__main__':
    fig = plt.figure()
    make_figure(fig, read_data())
    fig.savefig('result.png')
