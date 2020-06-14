#!/usr/bin/env python
# -*- coding: utf-8 -*-


import csv
import sys
import os.path
import logging
from datetime import datetime

from PyQt5.QtWidgets import QApplication, QProgressDialog, QDateTimeEdit, QLabel, QSizePolicy, QDialog, QAbstractScrollArea, QTableView, QAbstractItemView, QPushButton, QListView, QVBoxLayout,  QHBoxLayout, QMenu, QToolBar, QWidget, QDesktopWidget, QFileDialog, QListWidget, QAction, QMainWindow, QMessageBox, QStyle, QHeaderView
from PyQt5.QtGui import QFont, QIcon, QStandardItemModel, QStandardItem
from PyQt5.QtCore import QProcess, QDir, QEvent, pyqtSignal,  QThread, QCoreApplication, Qt, QTranslator, QObject,  QStringListModel, QTime


class ProgressDialog(QProgressDialog):
    def __init__(self, parent, title, command, args):
        super().__init__(title,  QCoreApplication.translate(
            'TaskProgressDialog', "cancel"), 0, 100, parent)
        self.setWindowModality(Qt.WindowModal)
        self.process = QProcess()
        logging.info("%s %s" % (command, ' '.join(args)))
        self.process.start(command, args)
        self.process.finished.connect(self.reject)
        self.process.readyReadStandardOutput.connect(self.addStdOut)
        self.process.readyReadStandardError.connect(self.addStdErr)
        self.canceled.connect(self.process.terminate)
        self.show()

    def addStdOut(self):
        logging.debug(bytes(self.process.readAllStandardOutput()).decode())

    def addStdErr(self):
        logging.error(bytes(self.process.readAllStandardError()).decode())


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
        self.toolbarNew.setIcon(QIcon("icons/new.svg"))
        self.toolbarNew.triggered.connect(self.onNew)
        toolbar.addAction(self.toolbarNew)

        self.toolbarEdit = QAction(self)
        self.toolbarEdit.setIcon(QIcon("icons/edit.svg"))
        self.toolbarEdit.triggered.connect(self.onEdit)
        toolbar.addAction(self.toolbarEdit)

        self.toolbarUp = QAction(self)
        self.toolbarUp.setIcon(QIcon("icons/arrow-up.svg"))
        self.toolbarUp.triggered.connect(self.onUp)
        toolbar.addAction(self.toolbarUp)

        self.toolbarDown = QAction(self)
        self.toolbarDown.setIcon(QIcon("icons/arrow-down.svg"))
        self.toolbarDown.triggered.connect(self.onDown)
        toolbar.addAction(self.toolbarDown)

        self.toolbarDelete = QAction(self)
        self.toolbarDelete.setIcon(QIcon("icons/ashbin.svg"))
        self.toolbarDelete.triggered.connect(self.onDelete)
        toolbar.addAction(self.toolbarDelete)

        self.toolbarRun = QAction(self)
        self.toolbarRun.setIcon(QIcon("icons/process.svg"))
        self.toolbarRun.triggered.connect(self.onRun)
        toolbar.addAction(self.toolbarRun)

        self.toolbarQuit = QAction(self)
        self.toolbarQuit.setIcon(QIcon("icons/quit.svg"))
        self.toolbarQuit.triggered.connect(self.close)
        toolbar.addAction(self.toolbarQuit)

        self.toolbarAbout = QAction(self)
        self.toolbarAbout.setIcon(QIcon("icons/about.svg"))
        self.toolbarAbout.triggered.connect(self.onAboutUs)
        toolbar.addAction(self.toolbarAbout)

    def initMenuBar(self):
        self.quitFileMenu = QAction(self)
        self.quitFileMenu.setIcon(QIcon("icons/quit.svg"))
        self.quitFileMenu.setShortcut('Ctrl+Q')
        self.quitFileMenu.triggered.connect(self.close)

        self.aboutHelpMenu = QAction(self)
        self.aboutHelpMenu.setIcon(QIcon("icons/about.svg"))
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
        self.taskTable = QTableView()
        self.taskTable.setSelectionBehavior(QTableView.SelectRows)
        self.box.addWidget(self.taskTable)

        self.taskModel = QStandardItemModel()
        self.taskModel.setHorizontalHeaderLabels(['file', 'begin', 'end'])
        self.taskTable.setModel(self.taskModel)

        # self.taskModel.appendRow(
        #     [QStandardItem("in.mp4"), QStandardItem("00:00:15"), QStandardItem("00:00:30")])
        # self.taskModel.appendRow(
        #     [QStandardItem("in.mp4"), QStandardItem("00:01:15"), QStandardItem("00:01:45")])
        # self.taskModel.appendRow(
        #     [QStandardItem("in.mp4"), QStandardItem("00:02:00"), QStandardItem("00:03:00")])

        header = self.taskTable.horizontalHeader()
        header.setSectionResizeMode(0, QHeaderView.Stretch)
        header.setSectionResizeMode(1, QHeaderView.ResizeToContents)
        header.setSectionResizeMode(2, QHeaderView.ResizeToContents)

    def initLogList(self):
        self.logTable = QTableView()
        self.logTable.setSelectionBehavior(QTableView.SelectRows)
        self.logTable.setSelectionMode(QAbstractItemView.ExtendedSelection)
        self.box.addWidget(self.logTable)

        self.logModel = QStandardItemModel()
        self.logModel.setHorizontalHeaderLabels(['created-at', 'message'])
        self.logTable.setModel(self.logModel)

        header = self.logTable.horizontalHeader()
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
        self.setWindowIcon(QIcon("icons/moive.svg"))
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

    # https://stackoverflow.com/questions/32604886/ffmpeg-concat-protocol-does-not-combine-video-files
    def onRun(self):
        if self.taskModel.rowCount() == 0:
            QMessageBox.critical(self, QCoreApplication.translate(
                'TaskTable', "error"), QCoreApplication.translate('TaskTable', "empty"))
            return
        name, _ = QFileDialog.getSaveFileName(self)
        if not name:
            return

        command = "ffmpeg"
        if os.name == 'nt':
            command = "%s\\ffmpeg\\bin\\ffmpeg.ext" % os.getenv("SystemDrive")

        files = []
        for it in range(self.taskModel.rowCount()):
            file = self.taskModel.data(self.taskModel.index(it, 0))
            begin = self.taskModel.data(self.taskModel.index(it, 1))
            end = self.taskModel.data(self.taskModel.index(it, 2))
            out = "%s.%d" % (name, it)
            if self.taskModel.rowCount() == 1:
                out = name
            args = ["-y", "-i", file, "-ss", begin,
                    "-to", end, "-c", "copy", "-bsf:v", "h264_mp4toannexb",
                    "-f", "mpegts", out]
            self.appendLog("%s %s" % (command, ' '.join(args)))
            dlg = ProgressDialog(self, out, command, args)
            dlg.exec()

            if dlg.process.exitStatus() != QProcess.NormalExit:
                QMessageBox.critical(self, QCoreApplication.translate(
                    'TaskTable', "error"), dlg.process.errorString())
                return
            files.append(out)

        if len(files) <= 1:
            self.appendLog("Done!")
            return
        args = ["-y", "-i", "concat:%s" % "|".join(files),
                "-c", "copy", "-brand", "mp42", "-bsf:a", "aac_adtstoasc", name]
        self.appendLog("%s %s" % (command, ' '.join(args)))
        dlg = ProgressDialog(self, out, command, args)
        dlg.exec()

        if dlg.process.exitStatus() != QProcess.NormalExit:
            QMessageBox.critical(self, QCoreApplication.translate(
                'TaskTable', "error"), dlg.process.errorString())
            return
        self.appendLog("Done!")

    def onNew(self):
        self.taskDialog()

    def onEdit(self):
        items = self.taskTable.selectedIndexes()
        if len(items) == 0:
            QMessageBox.critical(self, QCoreApplication.translate(
                'TaskTable', "error"), QCoreApplication.translate('TaskTable', "not-selected"))
            return

        self.taskDialog(items[0])

    def onUp(self):
        items = self.taskTable.selectedIndexes()
        if len(items) == 0:
            QMessageBox.critical(self, QCoreApplication.translate(
                'TaskTable', "error"), QCoreApplication.translate('TaskTable', "not-selected"))
            return
        row = items[0].row()
        if row == 0:
            return
        items = self.taskModel.takeRow(row)
        self.taskModel.insertRow(row-1, items)

    def onDown(self):
        items = self.taskTable.selectedIndexes()
        if len(items) == 0:
            QMessageBox.critical(self, QCoreApplication.translate(
                'TaskTable', "error"), QCoreApplication.translate('TaskTable', "not-selected"))
            return
        row = items[0].row()
        if row == self.taskModel.rowCount() - 1:
            return
        items = self.taskModel.takeRow(row)
        self.taskModel.insertRow(row + 1, items)

    def onDelete(self):
        items = self.taskTable.selectedIndexes()
        if len(items) == 0:
            QMessageBox.critical(self, QCoreApplication.translate(
                'TaskTable', "error"), QCoreApplication.translate('TaskTable', "not-selected"))
            return
        self.taskModel.removeRow(items[0].row())

    def taskDialog(self, it=None):
        dlg = TaskDialog(QCoreApplication.translate('TaskDialog', "title.new"))
        if it:
            row = it.row()
            dlg.file.setText(self.taskModel.data(self.taskModel.index(row, 0)))
            dlg.begin.setTime(QTime.fromString(
                self.taskModel.data(self.taskModel.index(row, 1)), TIME_FORMAT))
            dlg.end.setTime(QTime.fromString(
                self.taskModel.data(self.taskModel.index(row, 2)), TIME_FORMAT))

        dlg.show()
        dlg.exec()
        if (not dlg.file.text()) or dlg.begin.time() >= dlg.end.time():
            return
        logging.debug("file=%s begin=%s end=%s" % (
            dlg.file.text(), dlg.begin.time(), dlg.end.time()))
        item = [QStandardItem(dlg.file.text()), QStandardItem(
            dlg.begin.time().toString(TIME_FORMAT)), QStandardItem(dlg.end.time().toString(TIME_FORMAT))]
        if it:
            row = it.row()
            self.taskModel.setItem(row, 0, item[0])
            self.taskModel.setItem(row, 1, item[1])
            self.taskModel.setItem(row, 2, item[2])
        else:
            self.taskModel.appendRow(item)

    def appendLog(self, msg):
        self.logModel.appendRow(
            [QStandardItem(str(datetime.now())), QStandardItem(msg)])
        # self.logTable.resizeRowsToContents()

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
        QMessageBox.about(
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
        self.toolbarAbout.setText(
            QCoreApplication.translate('ToolBar', "about"))

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


TIME_FORMAT = "HH:mm:ss"


class TaskDialog(QDialog):
    def __init__(self, title):
        super().__init__()

        self.file = QPushButton()
        self.file.clicked.connect(self.onSelectFile)

        self.begin = QDateTimeEdit()
        self.begin.setDisplayFormat(TIME_FORMAT)

        self.end = QDateTimeEdit()
        self.end.setDisplayFormat(TIME_FORMAT)

        self.initUi()
        self.setWindowTitle(title)
        # self.setMinimumWidth(200)
        self.setWindowModality(Qt.ApplicationModal)

    def initUi(self):
        self.setSizePolicy(QSizePolicy.Expanding, QSizePolicy.Expanding)
        box = QVBoxLayout()
        box.addWidget(QLabel(QCoreApplication.translate('TaskTable', "file")))
        box.addWidget(self.file)

        box.addWidget(QLabel(QCoreApplication.translate('TaskTable', "begin")))
        box.addWidget(self.begin)

        box.addWidget(QLabel(QCoreApplication.translate('TaskTable', "end")))
        box.addWidget(self.end)

        submit = QPushButton(
            QCoreApplication.translate('TaskDialog', "submit"))
        submit.clicked.connect(self.onSubmit)
        box.addWidget(submit)

        self.setLayout(box)

    def onSelectFile(self):
        name, _ = QFileDialog.getOpenFileName(self)
        if name:
            self.file.setText(name)

    def onSubmit(self):
        if not self.file.text():
            QMessageBox.critical(self, QCoreApplication.translate(
                'TaskDialog', "error"), QCoreApplication.translate('TaskDialog', "select-file"))
            return
        if self.begin.time() >= self.end.time():
            QMessageBox.critical(self, QCoreApplication.translate(
                'TaskDialog', "error"), QCoreApplication.translate('TaskDialog', "bad-times"))
            return
        self.reject()


if __name__ == '__main__':
    logging.basicConfig(filename='{:%Y-%m-%d}.log'.format(datetime.now()),
                        format='%(asctime)s %(levelname)s: %(message)s',
                        level=logging.DEBUG)
    logging.info("start...")
    app = QApplication(sys.argv)
    # app.setStyle('Cleanlooks')  # 'Windows' Macintosh
    win = MainWindow()
    sys.exit(app.exec_())
