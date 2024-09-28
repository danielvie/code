import sys
from TaskView import TaskView
from TaskViewModel import TaskViewModel
from PyQt6.QtWidgets import QApplication

if __name__ == "__main__":

    # Create App
    app = QApplication(sys.argv)

    # Create ViewModel
    task_viewmodel = TaskViewModel()

    # Create and display the view
    view = TaskView(task_viewmodel)

    # Execute App
    sys.exit(app.exec())
