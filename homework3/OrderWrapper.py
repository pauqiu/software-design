class Order:
    def __init__(self, factory, item):
        self.factory = factory
        self.item = item

    def make_order(self):
        self.factory.make_order(self.item)