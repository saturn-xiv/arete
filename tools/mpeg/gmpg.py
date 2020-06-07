#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""Setup.

https://www.riverbankcomputing.com/static/Docs/PyQt5/
http://zetcode.com/gui/pyqt5/

install third packages:
$ sudo apt-get install ffmpeg

"""


import subprocess
import csv
import sys
import os.path
import logging
from datetime import datetime
from PyQt5.QtWidgets import QApplication, QMenu, QToolBar, QWidget, QDesktopWidget, QFileDialog, QListWidget, QAction, QMainWindow, QMessageBox, QStyle
from PyQt5.QtGui import QIcon
from PyQt5.QtCore import QDir, QEvent, QCoreApplication, QTranslator, QObject


class Slice:
    def __init__(self, order, file, begin, end):
        self.order = order
        self.file = file
        self.begin = begin
        self.end = end


def _join():
    pass


def _split(file, _from, to):
    pass


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.trans = QTranslator(self)
        self.initMenuBar()
        self.initToolBar()
        self.initUI()

        self.setLang('en_US')
        self.retranslateUi()

    def initToolBar(self):
        toolbar = QToolBar()
        self.addToolBar(toolbar)
        self.toolbarNew = QAction(self)
        self.toolbarNew.triggered.connect(self.close)
        toolbar.addAction(self.toolbarNew)

    def initMenuBar(self):
        self.quitFileMenu = QAction(self)
        self.quitFileMenu.setIcon(self.style().standardIcon(
            QStyle.SP_DialogCloseButton))
        self.quitFileMenu.setShortcut('Ctrl+Q')
        self.quitFileMenu.triggered.connect(self.close)

        self.aboutHelpMenu = QAction(self)
        self.aboutHelpMenu.setIcon(self.style().standardIcon(
            QStyle.SP_DialogHelpButton))
        self.aboutHelpMenu.setShortcut('Ctrl+A')
        self.aboutHelpMenu.triggered.connect(self.onAboutUs)

        enUSLang = QAction('English', self)
        enUSLang.setShortcut('Ctrl+E')
        enUSLang.setObjectName('en_US')
        enUSLang.triggered.connect(self.onSetLang)

        zhCNLang = QAction('简体中文', self)
        zhCNLang.setShortcut('Ctrl+Z')
        zhCNLang.setObjectName('zh_CN')
        zhCNLang.triggered.connect(self.onSetLang)

        menubar = self.menuBar()

        self.fileMenu = QMenu(self)
        self.fileMenu.addAction(self.quitFileMenu)
        menubar.addMenu(self.fileMenu)

        self.langMenu = QMenu(self)
        self.langMenu.addAction(enUSLang)
        self.langMenu.addAction(zhCNLang)
        menubar.addMenu(self.langMenu)

        self.helpMenu = QMenu(self)
        self.helpMenu.addAction(self.aboutHelpMenu)
        menubar.addMenu(self.helpMenu)

    def initUI(self):
        self.listWidget = QListWidget()
        self.setCentralWidget(self.listWidget)
        self.statusBar()

        # self.resize(800, 600)
        # self.move(QDesktopWidget().availableGeometry().center())
        self.showMaximized()
        self.show()

    def closeEvent(self, event):
        close = QMessageBox.question(self,
                                     QCoreApplication.translate(
                                         'ExitDialog', "title"),
                                     QCoreApplication.translate(
                                         'ExitDialog', "body"),
                                     QMessageBox.Yes | QMessageBox.No)
        if close == QMessageBox.Yes:
            logging.info("exit...")
            event.accept()
        else:
            event.ignore()

    def setLang(self, lng):
        logging.info("switch lang to %s" % lng)
        self.trans.load(lng, directory="locales")
        QApplication.instance().installTranslator(self.trans)
        # QApplication.instance().removeTranslator(self.trans)

    def onSetLang(self):
        self.setLang(self.sender().objectName())

    def changeEvent(self, event):
        if event.type() == QEvent.LanguageChange:
            self.retranslateUi()
        super(MainWindow, self).changeEvent(event)

    def onAboutUs(self):
        QMessageBox.information(
            self, "Arete", "https://github.com/saturn-xiv/arete/tree/master/tools/mpeg")

    def retranslateUi(self):
        self.toolbarNew.setText(QCoreApplication.translate('ToolBar', "new"))

        self.fileMenu.setTitle(
            QCoreApplication.translate('MenuBar', "file"))
        self.quitFileMenu.setText(
            QCoreApplication.translate('MenuBar', "file.quit"))
        self.langMenu.setTitle(
            QCoreApplication.translate('MenuBar', "lang"))
        self.helpMenu.setTitle(
            QCoreApplication.translate('MenuBar', "help"))
        self.aboutHelpMenu.setText(
            QCoreApplication.translate('MenuBar', "help.about"))

        self.setWindowTitle(QCoreApplication.translate('Window', "title"))


if __name__ == '__main__':
    logging.basicConfig(filename='{:%Y-%m-%d}.log'.format(datetime.now()),
                        format='%(asctime)s %(levelname)s: %(message)s',
                        level=logging.DEBUG)
    logging.info("start...")
    app = QApplication(sys.argv)
    win = MainWindow()
    sys.exit(app.exec_())
