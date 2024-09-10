from TaskModel import TaskModel
from PyQt6.QtCore import pyqtSignal, QObject

# ViewModel
class TaskViewModel(QObject):
    tasks_changed = pyqtSignal()
    show_progress_changed = pyqtSignal(int, str)
    hide_progress_changed = pyqtSignal()
    value = 0

    def __init__(self):
        super().__init__()
        self.model = TaskModel()

    def add_task(self, task):
        self.model.add_task(task)
        self.tasks_changed.emit()

        self.value += 10
        self.value = min(100, self.value)
        self.show_progress_changed.emit(self.value, f"indo .. {self.value}")

    def delete_task(self, task):
        self.model.delete_task(task)
        self.tasks_changed.emit()
        self.hide_progress_changed.emit()

    def get_tasks(self):
        return self.model.get_tasks()