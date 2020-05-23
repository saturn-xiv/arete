#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""Setup.

https://www.riverbankcomputing.com/static/Docs/PyQt5/

install third packages:
$ sudo apt-get install ffmpeg

"""


import subprocess
import csv
import sys
import os.path
from PyQt5.QtWidgets import QApplication, QWidget, QDesktopWidget, QFileDialog, QListWidget, QAction, QMainWindow
from PyQt5.QtGui import QIcon
from PyQt5.QtCore import QDir

# ffmpeg - i in .mp4 - ss 01: 01: 01 - to 02: 02: 02 - c copy out.mp4
# ffmpeg - f concat - i filelist.txt - c copy out.mp4


def _split(file, _from, to):
    pass


class MainWindow(QMainWindow):

    def __init__(self):
        super().__init__()

        self.initUI()

    def initUI(self):

        self.listWidget = QListWidget()
        self.setCentralWidget(self.listWidget)
        self.statusBar()

        openFile = QAction(QIcon('open.png'), 'Open', self)
        openFile.setShortcut('Ctrl+O')
        openFile.setStatusTip('Open a config file')
        openFile.triggered.connect(self.showDialog)

        quitFile = QAction(QIcon('quit.png'), 'Quit', self)
        quitFile.setShortcut('Ctrl+Q')
        quitFile.setStatusTip('Close window')
        quitFile.triggered.connect(self.close)

        menubar = self.menuBar()
        fileMenu = menubar.addMenu('&File')
        fileMenu.addAction(openFile)
        fileMenu.addAction(quitFile)

        self.resize(800, 600)
        self.move(QDesktopWidget().availableGeometry().center())
        self.setWindowTitle('MP4 cutter & linker')
        self.show()

    def showDialog(self):

        fname = QFileDialog.getOpenFileName(self, 'Open file', QDir.homePath())

        if fname[0]:
            self.listWidget.clear()
            with open(fname[0], 'r') as file:
                rdr = csv.reader(file)
                for row in rdr:
                    if len(row) == 3:
                        self.listWidget.addItem("INFO: find task file(%s) from(%s) to(%s)" % (
                            row[0], row[1], row[2]))
                        if not os.path.exists(row[0]):
                            self.listWidget.addItem(
                                "ERROR: can't find file %s" % row[0])
                    else:
                        self.listWidget.addItem("WARN: ignore line %s" % row)


if __name__ == '__main__':
    app = QApplication(sys.argv)
    win = MainWindow()
    sys.exit(app.exec_())
