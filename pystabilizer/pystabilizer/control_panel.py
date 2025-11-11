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
        self._stabilizer.connection_state_update.connect(
            self._on_connection_state_changed
        )

    @pyqtSlot(StabilizerConnectionState)
    def _on_connection_state_changed(self, state):

        match state:
            case StabilizerConnectionState.CONNECTED:
                self.connect_btn.setText("Disconnect")
                self.status_lbl.setText(
                    "Connected to Stabilizer"
                )

            case StabilizerConnectionState.CONNECTING:
                self.connect_btn.setText("Stop")
                self.status_lbl.setText("Connecting...")

            case StabilizerConnectionState.DISCONNECTED:
                self.connect_btn.setText("Connect")
                self.status_lbl.setText("Disconnected")

    @asyncSlot()
    async def on_connect_btn_clicked(self):
        match self._stabilizer.connection_state:
            case StabilizerConnectionState.DISCONNECTED:
                self._connecting_task = asyncio.current_task()
                self._stabilizer.connection_state = StabilizerConnectionState.CONNECTING
                print("starting to connect")
                await self._stabilizer.start_session()
                print("connected")
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
