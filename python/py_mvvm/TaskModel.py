# Model
class TaskModel:
    def __init__(self):
        self._tasks = []

    def add_task(self, task):
        self._tasks.append(task)

    def delete_task(self, task):
        self._tasks.remove(task)

    def clear_task(self):
        self._tasks.clear()

    def get_tasks(self):
        return self._tasks