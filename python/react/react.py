class InputCell:
    def __init__(self, initial_value):
        self._value = initial_value 
        self.subscribers = set()

    def subscribe(self, compute_cell):
        self.subscribers.add(compute_cell)
        return self

    def dispatch(self):
        for s in self.subscribers:
            s.compute_value()

    @property
    def value(self):
        return self._value
    
    @value.setter
    def value(self, value):
        self._value = value
        self.dispatch()

# TODO: compute cell needs to dispatch when it's value changes
# currently only 12/14 tests pass because of this
class ComputeCell:
    def __init__(self, inputs, compute_function):
        self.inputs = [i.subscribe(self) for i in inputs] 
        self.compute_function = compute_function
        self.callback = set()
        self.subscribers = set()

    def subscribe(self, compute_cell):
        self.subscribers.add(compute_cell)
        return self

    def dispatch(self):
        for s in self.subscribers:
            s.compute_value()

    def add_callback(self, callback):
        self.callback.add(callback) 

    def remove_callback(self, callback):
        if callback in self.callback:
            self.callback.remove(callback) 
    
    @property
    def value(self):
        return self.compute_value()

    def compute_value(self):
        values = [i.value for i in self.inputs]
        value = self.compute_function(values) 
        self.call_callbacks(value)
        return value

    def call_callbacks(self, value):
        for cb in self.callback: 
            cb(value)
