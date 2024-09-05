from TaskModel import TaskModel
from PyQt5.QtCore import pyqtSignal, QObject

# ViewModel
class TaskViewModel(QObject):
    tasks_changed = pyqtSignal()

    def __init__(self):
        super().__init__()
        self.model = TaskModel()

    def add_task(self, task):
        self.model.add_task(task)
        self.tasks_changed.emit()

    def delete_task(self, task):
        self.model.delete_task(task)
        self.tasks_changed.emit()

    def get_tasks(self):
        return self.model.get_tasks()