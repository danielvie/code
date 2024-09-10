import sys
from TaskView import TaskView
from TaskViewModel import TaskViewModel
from PyQt6.QtWidgets import QApplication

if __name__ == "__main__":

    app = QApplication(sys.argv)

    # Create ViewModel
    task_viewmodel = TaskViewModel()

    # Create and display the view
    view = TaskView(task_viewmodel)

    sys.exit(app.exec())
