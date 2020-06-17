class TreeNode:
    def __init__(self, data, left=None, right=None):
        self.data = data
        self.left = left
        self.right = right

    def insert(self, data):
        if int(data) <= int(self.data):
            if self.left == None:
                self.left = TreeNode(data)
            else:
                self.left.insert(data)
        elif int(data) > int(self.data):
            if self.right == None:
                self.right = TreeNode(data)
            else:
                self.right.insert(data)
                
    def sort(self, ls = None):
        if ls == None:
            ls = []
        if self.left != None:
            self.left.sort(ls)
        ls.append(self.data)
        if self.right != None:
            self.right.sort(ls)     
        return ls

    def __str__(self):
        fmt = 'TreeNode(data={}, left={}, right={})'
        return fmt.format(self.data, self.left, self.right)


class BinarySearchTree:
    def __init__(self, tree_data):
        self.tree = TreeNode(tree_data[0])
        if len(tree_data) > 1:
            for data in tree_data[1:]:
                self.tree.insert(data)

    def data(self):
        return self.tree
    
    def sorted_data(self):
        return self.tree.sort()
