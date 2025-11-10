from enum import Enum
from PyQt6.QtCore import pyqtSignal, QObject, pyqtSlot
from pystabilizer.aioclient import AsyncioClient
from qasync import asyncSlot

class StabilizerConnectionState(Enum):
    DISCONNECTED = "disconnected"
    CONNECTING = "connecting"
    CONNECTED = "connected"

class Stabilizer(QObject):
    connection_state = StabilizerConnectionState

    def __init__(self, parent):
        super().__init__(parent)

        self.connection_state = StabilizerConnectionState.DISCONNECTED
        self._client = AsyncioClient()

    async def start_session(self, host, port):
        await self._client.connect(host, port)
        # self.hw_rev = await self._client.get_hwrev()

    @asyncSlot()
    async def end_session(self):
        # self.stop_watching()
        await self.disconnect_cb()