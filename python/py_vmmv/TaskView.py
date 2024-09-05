from PyQt5.QtWidgets import QWidget, QVBoxLayout, QLineEdit, QListWidget, QPushButton

# View
class TaskView(QWidget):
    def __init__(self, viewmodel):
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

        # Delete task button
        self.delete_button = QPushButton("Delete Task", self)
        self.delete_button.clicked.connect(self.delete_task)
        layout.addWidget(self.delete_button)

        # Task list
        self.task_list = QListWidget(self)
        layout.addWidget(self.task_list)

        # Binding ViewModel to View
        self.viewmodel.tasks_changed.connect(self.update_view)

        self.setLayout(layout)
        self.setWindowTitle("MVVM Task Manager")
        self.show()

    def add_task(self):
        task = self.task_input.text()
        if task:
            self.viewmodel.add_task(task)
            self.task_input.clear()

    def delete_task(self):
        task = self.task_list.currentItem().text()
        if task:
            self.viewmodel.delete_task(task)
            self.task_input.clear()

    def update_view(self):
        self.task_list.clear()
        self.task_list.addItems(self.viewmodel.get_tasks())
