import threading
import queue

class FoodService:
    def __init__(self) -> None:
        self.orders = []
        self.threads = []

    def add_order(self, order):
        self.orders.append(order)

    def deactivate_threads(self):
        for t in self.threads:
            t.join()

    def start_working(self):
        # Thread-safe queue for dynamically splitting tasks
        tasks = queue.Queue()
        for item in self.orders:
            tasks.put(item)

        # Setting up two threads
        for i in range (2):
            cook = threading.Thread(target=self.process_orders, args=(tasks,))
            cook.start()
            self.threads.append(cook)

        # This method waits for all tasks to be taken out of the queue
        tasks.join()

        self.deactivate_threads()

        print("[System] All tasks are done and all cooks went home!")

    def process_orders(self, tasks):
        while True:
            # Check if there are tasks left in the queue
            try:
                item = tasks.get(timeout=1)
                item.make_order()

                # Signal completed task
                tasks.task_done()
            except queue.Empty:
                break
        