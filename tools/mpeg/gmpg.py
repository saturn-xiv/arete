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
from PyQt5.QtWidgets import QApplication, QAbstractScrollArea, QTableView, QAbstractItemView, QPushButton, QListView, QHBoxLayout, QMenu, QToolBar, QWidget, QDesktopWidget, QFileDialog, QListWidget, QAction, QMainWindow, QMessageBox, QStyle, QHeaderView
from PyQt5.QtGui import QIcon, QStandardItemModel, QStandardItem
from PyQt5.QtCore import QDir, QEvent,  QCoreApplication, QTranslator, QObject,  QStringListModel


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
        self.initCentralPanel()
        self.initUI()

        self.setLang('en_US')
        self.retranslateUi()

    def initToolBar(self):
        toolbar = QToolBar()
        toolbar.setMovable(False)
        self.addToolBar(toolbar)

        self.toolbarNew = QAction(self)
        self.toolbarNew.setIcon(self.style().standardIcon(
            QStyle.SP_FileDialogNewFolder))
        self.toolbarNew.triggered.connect(self.close)
        toolbar.addAction(self.toolbarNew)

        self.toolbarEdit = QAction(self)
        self.toolbarEdit.setIcon(self.style().standardIcon(
            QStyle.SP_LineEditClearButton))
        self.toolbarEdit.triggered.connect(self.close)
        toolbar.addAction(self.toolbarEdit)

        self.toolbarUp = QAction(self)
        self.toolbarUp.setIcon(self.style().standardIcon(
            QStyle.SP_ArrowUp))
        self.toolbarUp.triggered.connect(self.close)
        toolbar.addAction(self.toolbarUp)

        self.toolbarDown = QAction(self)
        self.toolbarDown.setIcon(self.style().standardIcon(
            QStyle.SP_ArrowDown))
        self.toolbarDown.triggered.connect(self.close)
        toolbar.addAction(self.toolbarDown)

        self.toolbarDelete = QAction(self)
        self.toolbarDelete.setIcon(self.style().standardIcon(
            QStyle.SP_DialogDiscardButton))
        self.toolbarDelete.triggered.connect(self.close)
        toolbar.addAction(self.toolbarDelete)

        self.toolbarRun = QAction(self)
        self.toolbarRun.setIcon(self.style().standardIcon(
            QStyle.SP_MediaPlay))
        self.toolbarRun.triggered.connect(self.onRun)
        toolbar.addAction(self.toolbarRun)

        self.toolbarQuit = QAction(self)
        self.toolbarQuit.setIcon(self.style().standardIcon(
            QStyle.SP_DialogCloseButton))
        self.toolbarQuit.triggered.connect(self.close)
        toolbar.addAction(self.toolbarQuit)

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

    def initTaskTable(self):
        taskTable = QTableView()
        self.box.addWidget(taskTable)

        self.taskModel = QStandardItemModel()
        self.taskModel.setHorizontalHeaderLabels(['file', 'begin', 'end'])
        taskTable.setModel(self.taskModel)

        header = taskTable.horizontalHeader()
        header.setSectionResizeMode(0, QHeaderView.Stretch)
        header.setSectionResizeMode(1, QHeaderView.ResizeToContents)
        header.setSectionResizeMode(2, QHeaderView.ResizeToContents)

    def initLogList(self):
        logTable = QTableView()
        logTable.setSelectionMode(QAbstractItemView.ExtendedSelection)
        self.box.addWidget(logTable)

        self.logModel = QStandardItemModel()
        self.logModel.setHorizontalHeaderLabels(['created-at', 'message'])
        logTable.setModel(self.logModel)

        header = logTable.horizontalHeader()
        header.setSectionResizeMode(0, QHeaderView.ResizeToContents)
        header.setSectionResizeMode(1, QHeaderView.Stretch)

    def initCentralPanel(self):
        central = QWidget(self)
        self.box = QHBoxLayout(central)
        self.initTaskTable()
        self.initLogList()
        self.setCentralWidget(central)

    def initUI(self):

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

    def onRun(self):
        self.appendLog("todo")

    def appendLog(self, msg):
        self.logModel.appendRow(
            [QStandardItem(str(datetime.now())), QStandardItem(msg)])

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
        self.toolbarUp.setText(QCoreApplication.translate('ToolBar', "up"))
        self.toolbarDown.setText(QCoreApplication.translate('ToolBar', "down"))
        self.toolbarDelete.setText(
            QCoreApplication.translate('ToolBar', "delete"))
        self.toolbarRun.setText(QCoreApplication.translate('ToolBar', "run"))
        self.toolbarEdit.setText(QCoreApplication.translate('ToolBar', "edit"))
        self.toolbarQuit.setText(QCoreApplication.translate('ToolBar', "quit"))

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

        self.logModel.setHorizontalHeaderLabels([
            QCoreApplication.translate('LogTable', "created-at"),
            QCoreApplication.translate('LogTable', "message")
        ])
        self.taskModel.setHorizontalHeaderLabels([
            QCoreApplication.translate('TaskTable', "file"),
            QCoreApplication.translate('TaskTable', "begin"),
            QCoreApplication.translate('TaskTable', "end")
        ])

        self.setWindowTitle(QCoreApplication.translate('Window', "title"))


if __name__ == '__main__':
    logging.basicConfig(filename='{:%Y-%m-%d}.log'.format(datetime.now()),
                        format='%(asctime)s %(levelname)s: %(message)s',
                        level=logging.DEBUG)
    logging.info("start...")
    app = QApplication(sys.argv)
    app.setStyle('Macintosh')  # 'Windows'
    win = MainWindow()
    sys.exit(app.exec_())
