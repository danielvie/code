class Model:
    def __init__(self) -> None:
        self._tasks: list[str] = []

    def add_task(self, task: str) -> None:
        self._tasks.append(task)

    def delete_task(self, task: str) -> None:
        for t in self._tasks:
            if t == task:
                self._tasks.remove(t)

    def get_tasks(self) -> list[str]:
        return self._tasks