import asyncio
import logging
from enum import Enum
from PyQt6.QtCore import pyqtSignal, QObject, pyqtSlot
from qasync import asyncSlot
from pystabilizer.aioclient import AsyncioClient
from pystabilizer.property import Property, PropertyMeta

class StabilizerConnectionState(Enum):
    DISCONNECTED = "disconnected"
    CONNECTING = "connecting"
    CONNECTED = "connected"

class Stabilizer(QObject, metaclass=PropertyMeta):
    connection_state = Property(StabilizerConnectionState)
    report = Property(str)

    def __init__(self, parent):
        super().__init__(parent)

        self.connection_state = StabilizerConnectionState.DISCONNECTED
        self._client = AsyncioClient()
        self._update_params_task = None

    async def start_session(self, host="192.168.1.33", port=5678):
        await self._client.connect(host, port)

    @asyncSlot()
    async def end_session(self):
        self.stop_watching()
        await self._client.disconnect()

    def start_watching(self):
        self._watch_task = asyncio.create_task(self.run())
    
    def stop_watching(self):
        if self._watch_task is not None:
            self._watch_task.cancel()
            self._watch_task = None
            self._update_params_task.cancel()
            self._update_params_task = None
    
    async def run(self):
        self._update_params_task = asyncio.create_task(self.update_params())
        while True:
            if self._update_params_task.done():
                try:
                    self._update_params_task.result()
                except OSError:
                    logging.error(
                        "Encountered an error while polling for information from Thermostat.",
                        exc_info=True,
                    )
                    await self.end_session()
                    self.connection_state = StabilizerConnectionState.DISCONNECTED
                    return
                self._update_params_task = asyncio.create_task(self.update_params())
            await asyncio.sleep(1)
    
    async def update_params(self):
        print("start updating params")
        (
            self.report,
        ) = await asyncio.gather(
            self._client.get_report(),
        )
        print("finished updating params")