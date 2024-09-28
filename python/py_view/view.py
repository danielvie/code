from PyQt6.QtWidgets import QWidget, QHBoxLayout, QVBoxLayout, \
                            QLineEdit, QListWidget, \
                            QPushButton, QProgressBar
from view_model import TaskViewModel

# View
class TaskView(QWidget):
    def __init__(self, viewmodel: TaskViewModel):
        super().__init__()
        self.viewmodel = viewmodel
        self.initUI()

    def initUI(self):
        layout = QVBoxLayout()

        # Task input
        self.task_input = QLineEdit(self)
        self.task_input.setPlaceholderText("Enter a new task")
        layout.addWidget(self.task_input)

        # Add task button
        self.add_button = QPushButton("Add Task", self)
        self.add_button.clicked.connect(self.add_task)
        layout.addWidget(self.add_button)

        # ButtonGroup
        button_layout = QHBoxLayout()

        # Delete task button
        self.delete_button = QPushButton("Delete Task", self)
        self.delete_button.clicked.connect(self.delete_task)

        self.clear_button = QPushButton("Clear Task", self)
        self.clear_button.clicked.connect(self.clear_task)

        button_layout.addWidget(self.delete_button)
        button_layout.addWidget(self.clear_button)
        layout.addLayout(button_layout)

        # Task list
        self.task_list = QListWidget(self)
        layout.addWidget(self.task_list)

        # Task list
        self.progress = QProgressBar(self)
        self.progress.setGeometry(50,50, 200, 30)
        self.progress.setValue(100)
        self.progress.setVisible(False)
        layout.addWidget(self.progress)

        # Binding ViewModel to View
        self.viewmodel.tasks_changed.connect(self.update_view)

        # Binding events to View
        self.viewmodel.show_progress_changed.connect(self.show_progress)
        self.viewmodel.hide_progress_changed.connect(self.hide_progress)

        self.setLayout(layout)
        self.setWindowTitle("Task Manager")
        self.show()

    def show_progress(self, value, message):
        self.progress.setVisible(True)
        self.progress.setValue(value)
        print(message)

    def hide_progress(self):
        self.progress.setVisible(False)

    def add_task(self):
        task = self.task_input.text()
        if task:
            self.viewmodel.add_task(task)
            self.task_input.clear()

    def delete_task(self):
        if self.task_list.currentItem() == None:
            return

        task = self.task_list.currentItem().text()
        if task:
            self.viewmodel.delete_task(task)
            self.task_input.clear()

    def clear_task(self):
        self.viewmodel.clear_task()
        self.task_input.clear()

    def update_view(self):
        self.task_list.clear()
        self.task_list.addItems(self.viewmodel.get_tasks())
