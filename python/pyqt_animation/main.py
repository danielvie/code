#!/usr/bin/python

from PyQt6.QtWidgets import *
from PyQt6.QtCore import *
import sys

class Example(QWidget):

    def __init__(self):
        super().__init__()

        self.initUI()

    def initUI(self):

        layout = QVBoxLayout()
        layout.setAlignment(Qt.AlignmentFlag.AlignTop)

        self.button = QPushButton("Start")
        self.button.clicked.connect(self.doAnim)
        self.button.move(30, 30)
        
        layout.addWidget(self.button)

        self.frame = QPushButton(self)
        # self.frame.setFrameStyle(QFrame.Shape.Panel | QFrame.Shadow.Raised)
        self.frame.setGeometry(100, 50, 100, 100)
        

        self.frame.setStyleSheet("""
        QPushButton {
            background-color: #006ca8
        }
""")

        self.setGeometry(300, 300, 380, 300)
        self.setWindowTitle('Animation')
        
        self.setLayout(layout)
        self.show()

    def doAnim(self):

        self.anim = QPropertyAnimation(self.frame, b"geometry")
        self.anim.setDuration(500)
        self.anim.setStartValue(QRect(100, 50, 100, 100))
        self.anim.setEndValue(QRect(100, 50, 200, 200))
        self.anim.start()


def main():

    app = QApplication([])
    _ = Example()
    sys.exit(app.exec())


if __name__ == "__main__":
    main()