# /usr/bin/env python3

import argparse
import json
import os
import shutil
import subprocess

from PySide6.QtCore import Qt, QProcess, QTimer
from PySide6.QtWidgets import (
    QApplication,
    QMainWindow,
    QGraphicsView,
    QGraphicsScene,
    QGraphicsRectItem,
    QWidget,
    QFormLayout,
    QDockWidget,
    QLineEdit,
    QSpinBox,
    QDoubleSpinBox,
    QCheckBox,
)
from PySide6.QtGui import (
    QPainter,
    QPen,
    QKeyEvent,
    QWheelEvent,
    QShowEvent,
    QColor,
    QPixmap,
)
from PySide6.QtSvgWidgets import QGraphicsSvgItem


class Skv(QMainWindow):
    SLOW_TIMEOUT = 5000
    FAST_TIMEOUT = 1500

    def __init__(self, command: list[str]):
        super().__init__()

        self.command = command
        self.svg = SvgView(self)

        self.parms_dock = None
        self.setCentralWidget(self.svg)

        self.timer = QTimer(self)
        self.timer.setInterval(self.SLOW_TIMEOUT)
        self.timer.timeout.connect(self.fireCommand)

        self.process = None
        self.loaded_svg_path = None

        self.manifest = None
        self.values = {}

        self.fireCommand()

        self.timer.start()
        self.status("Keep sketching!")
        self.svg.fit()

    def status(self, msg: str):
        self.statusBar().showMessage(msg, 1000)

    def fireCommand(self):
        if self.process:
            return

        self.timer.stop()
        self.process = QProcess(self)
        self.process.setEnvironment(QProcess.systemEnvironment() + ["SKV_VIEWER=1"])

        self.process.errorOccurred.connect(self.onCommandError)
        self.process.finished.connect(self.onCommandFinished)

        command = self.command[:]
        for param, value in self.values.items():
            command.append("--" + param)
            command.append(json.dumps(value))

        print(command)
        self.process.start(command[0], command[1:])
        self.timer.start()

    def onCommandError(self, err: QProcess.ProcessError):
        print("error running command", err)
        self.process = None

    def onCommandFinished(self, exit: int):
        if exit != 0:
            print("command returned {}", exit)
            self.process = None
            return

        manifest = None
        output = None

        data = bytearray(self.process.readAllStandardOutput())
        for l in data.splitlines():
            if not l.startswith(b"#SKV_VIEWER_COMMAND "):
                print(bytearray(l).decode("utf-8"))
                continue

            l = l.removeprefix(b"#SKV_VIEWER_COMMAND ")
            key, value = l.split(b"=", 1)
            if key == b"MANIFEST":
                print(value)
                manifest = json.loads(value)
            elif key == b"SVG":
                output = value
            else:
                print("unknown command", key)

        print(bytearray(self.process.readAllStandardError()).decode("utf-8"))

        if self.manifest != manifest:
            self.manifest = manifest
            self.rebuildParmsWidget()

        if output:
            self.loaded_svg_path = output.decode("utf-8")
            self.setWindowTitle(self.loaded_svg_path)
            self.svg.load(self.loaded_svg_path)
        else:
            self.loaded_svg_path = None

        self.process = None

    def rebuildParmsWidget(self):
        if self.parms_dock:
            self.removeDockWidget(self.parms_dock)

        self.parms_dock = QDockWidget("Parameters", self)

        self.values = {k: v for k, v in self.values.items() if k in self.manifest}

        widg = QWidget(self)
        layout = QFormLayout(widg)
        for param, schema in self.manifest.items():
            if param not in self.values:
                self.values[param] = schema["default"]

            value = self.values[param]

            def edit_over(p, v):
                self.values[p] = v
                self.fireCommand()

            ty = schema["type"]
            if ty == "string":
                w = QLineEdit(self)
                w.setText(value)
                w.editingFinished.connect(lambda p=param: edit_over(p, w.text()))
            elif ty == "int":
                w = QSpinBox(self)
                w.setMinimum(max(-(2**31), schema["min"]))
                w.setMaximum(min(2**31 - 1, schema["max"]))
                w.setValue(value)
                w.valueChanged.connect(lambda v, p=param: edit_over(p, v))
            elif ty == "double":
                w = QDoubleSpinBox(self)
                w.setMinimum(float(schema["min"]))
                w.setMaximum(float(schema["max"]))
                w.setValue(value)
                w.valueChanged.connect(lambda v, p=param: edit_over(p, v))
            elif ty == "bool":
                w = QCheckBox(self)
                w.setChecked(value)
                w.stateChanged.connect(lambda t, p=param: edit_over(p, t != 0))
            else:
                print("unsupported param type", ty)
                continue

            layout.addRow(param, w)
        self.parms_dock.setWidget(widg)

        self.addDockWidget(Qt.RightDockWidgetArea, self.parms_dock)

    def keyPressEvent(self, event: QKeyEvent):
        if event.key() == Qt.Key.Key_Escape:
            self.close()
            return

        if event.key() == Qt.Key_0:
            self.svg.fit()
            return

        if event.key() == Qt.Key_Minus:
            self.svg.zoomBy(0.5)
            return

        if event.key() == Qt.Key_R:
            self.svg.rotate(90)
            self.svg.fit()
            return

        if event.key() == Qt.Key_P:
            # TODO: how to take width/height of the output image?
            # as of now they're the same as of the svg dimensions, but it feels
            # wrong. Probably it would make sense to add it as a metadata in the
            # SVG in some way.
            self.status("exporting png...")
            pngdir = os.path.abspath(os.path.join(os.getcwd(), "png"))
            outfile = os.path.join(
                pngdir,
                os.path.splitext(os.path.basename(self.loaded_svg_path))[0] + ".png",
            )
            os.makedirs(pngdir, exist_ok=True)
            subprocess.check_call(
                [
                    "inkscape",
                    "-o",
                    outfile,
                    self.loaded_svg_path,
                ]
            )
            self.status("png exported")
            return

        if event.key() == Qt.Key_Space:
            self.timer.stop()
            self.status("Saving svg")
            outdir = os.path.abspath(os.path.join(os.getcwd(), "liked"))
            outfile = os.path.join(outdir, os.path.basename(self.loaded_svg_path))
            os.makedirs(outdir, exist_ok=True)
            shutil.copyfile(self.loaded_svg_path, outfile)
            self.optimizeSvg(outfile)
            self.status(f"Svg saved")
            self.timer.start()
            return

        if event.key() == Qt.Key_F:
            self.timer.stop()
            timeout = (
                self.SLOW_TIMEOUT
                if event.modifiers() & Qt.ShiftModifier
                else self.FAST_TIMEOUT
            )
            self.timer.setInterval(timeout)
            self.timer.start()
            return

        return super().keyPressEvent(event)

    def optimizeSvg(self, s: str):
        try:
            subprocess.check_call(
                [
                    "vpype",
                    "read",
                    s,
                    "linesimplify",
                    "linemerge",
                    "linesort",
                    "write",
                    s,
                ]
            )
        except subprocess.CalledProcessError:
            pass


class SvgView(QGraphicsView):
    def __init__(self, parent: QMainWindow):
        super().__init__(parent)

        self.setScene(QGraphicsScene(self))
        self.setTransformationAnchor(QGraphicsView.AnchorUnderMouse)
        self.setDragMode(QGraphicsView.ScrollHandDrag)
        self.setViewportUpdateMode(QGraphicsView.FullViewportUpdate)

        self.setRenderHint(QPainter.Antialiasing, True)

        # scrollbars prevent fitInView to work properly, disable them...
        self.setHorizontalScrollBarPolicy(Qt.ScrollBarAlwaysOff)
        self.setVerticalScrollBarPolicy(Qt.ScrollBarAlwaysOff)

        tile_pix = QPixmap(32, 32)
        tile_pix.fill(Qt.white)
        tile_p = QPainter(tile_pix)
        tile_p.fillRect(0, 0, 16, 16, QColor(220, 220, 220))
        tile_p.fillRect(16, 16, 16, 16, QColor(220, 220, 220))
        tile_p.end()

        self.setBackgroundBrush(tile_pix)

    def load(self, path: str):
        s = self.scene()
        s.clear()

        svg_item = QGraphicsSvgItem()
        r = svg_item.renderer()
        assert r.load(path)
        r.setAspectRatioMode(Qt.KeepAspectRatio)
        svg_item.setElementId("")  # force bbox update
        svg_item.setZValue(0)

        bg_item = QGraphicsRectItem(svg_item.boundingRect())
        bg_item.setBrush(Qt.white)
        bg_item.setPen(Qt.NoPen)
        bg_item.setZValue(-1)

        outline_item = QGraphicsRectItem(svg_item.boundingRect().translated(3, 3))
        outline = QPen(Qt.black, 1)
        outline.setCosmetic(True)
        outline_item.setPen(Qt.NoPen)
        outline_item.setBrush(Qt.gray)
        outline_item.setZValue(-2)

        s.addItem(svg_item)
        s.addItem(bg_item)
        s.addItem(outline_item)

    def fit(self):
        s = self.scene()
        self.setSceneRect(s.itemsBoundingRect())
        self.fitInView(s.itemsBoundingRect(), Qt.KeepAspectRatio)

    def zoomBy(self, s: float):
        zoom = self.transform().m11()
        if (s < 1 and zoom < 0.1) or (s > 1 and zoom > 10):
            return

        self.scale(s, s)

    def wheelEvent(self, event: QWheelEvent):
        self.zoomBy(1.2 ** (event.angleDelta().y() / 240.0))

    def showEvent(self, event: QShowEvent) -> None:
        super().showEvent(event)
        self.fit()


def main():
    p = argparse.ArgumentParser()
    p.add_argument("command", nargs="+")
    args = p.parse_args()

    app = QApplication()

    mainw = Skv(args.command)
    mainw.resize(1024, 1024)
    mainw.show()

    app.exec()


if __name__ == "__main__":
    main()
