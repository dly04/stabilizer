from PyQt6 import QtWidgets, QtGui, uic
from PyQt6.QtCore import pyqtSlot
import qasync
from qasync import asyncSlot, asyncClose
import importlib.resources
import asyncio

class MainWindow(QtWidgets.QMainWindow):
    NUM_CHANNELS = 2

    def __init__(self, args=None):
        super().__init__()

        ui_file_path = importlib.resources.files("pystabilizer.gui.view").joinpath("MainWindow.ui")
        uic.loadUi(ui_file_path, self)


async def coro_main():
    app_quit_event = asyncio.Event()

    app = QtWidgets.QApplication.instance()
    app.aboutToQuit.connect(app_quit_event.set)

    main_window = MainWindow()
    main_window.show()

    await app_quit_event.wait()

def main():
    qasync.run(coro_main())

if __name__ == "__main__":
    main()
