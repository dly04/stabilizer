from PyQt6 import QtWidgets, QtGui, uic
from PyQt6.QtCore import pyqtSlot
import qasync
from qasync import asyncSlot, asyncClose
import importlib.resources
import asyncio
from pystabilizer.stabilizer import Stabilizer, StabilizerConnectionState

class MainWindow(QtWidgets.QMainWindow):
    NUM_CHANNELS = 2

    def __init__(self, args=None):
        super().__init__()

        ui_file_path = importlib.resources.files("pystabilizer").joinpath("MainWindow.ui")
        uic.loadUi(ui_file_path, self)

        self._stabilizer = Stabilizer(self)
        self._connecting_task = None

    @asyncSlot()
    async def on_connect_btn_clicked(self):
        match self._stabilizer.connection_state:
            case StabilizerConnectionState.DISCONNECTED:
                self._connecting_task = asyncio.current_task()
                self._stabilizer.connection_state = StabilizerConnectionState.CONNECTING
                await self._stabilizer.start_session(
                    host=self.connection_details_menu.host_set_line.text(),
                    port=self.connection_details_menu.port_set_spin.value(),
                )
                self._connecting_task = None
                self._stabilizer.connection_state = StabilizerConnectionState.CONNECTED
                self._stabilizer.start_watching()

            case StabilizerConnectionState.CONNECTING:
                self._connecting_task.cancel()
                self._connecting_task = None
                await self._stabilizer.end_session()
                self._stabilizer.connection_state = (
                    StabilizerConnectionState.DISCONNECTED
                )

            case StabilizerConnectionState.CONNECTED:
                await self._stabilizer.end_session()
                self._stabilizer.connection_state = (
                    StabilizerConnectionState.DISCONNECTED
                )

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
