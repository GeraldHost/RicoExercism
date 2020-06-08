use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList::<T> {
            head: None
        }
    }

    pub fn len(&self) -> usize {
        let mut count: usize = 0;
        let mut data = &self.head;
        while let Some(node) = data {
            count+=1;
            data = &node.next;
        }
        count
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node::<T> {
            data: _element,
            next: self.head.take()
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(mut node) => {
                self.head = node.next.take();
                return Some(node.data);
            },
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.data),
            None => None
        }
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut simple_linked_list = Self::new();
        while let Some(node) = self.pop() {
            simple_linked_list.push(node);
        }
        simple_linked_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut simple_linked_list = Self::new();
        for i in _iter {
            simple_linked_list.push(i);
        }
        simple_linked_list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(node) = self.pop() {
            vec.push(node);
        }
        vec.reverse();
        vec
    }
}
