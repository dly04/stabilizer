from enum import Enum
from PyQt6.QtCore import pyqtSignal, QObject, pyqtSlot
from pystabilizer.aioclient import AsyncioClient

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
