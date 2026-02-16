use std::marker::PhantomData;
pub struct SimpleLinkedList<T> {
    // Delete this field
     list: Vec<T>,
    // dummy is needed to avoid unused parameter error during compilation
    dummy: PhantomData<T>,
    
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            dummy: PhantomData,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        if self.list.len() == 0 {
            return true
        }else{
            return false
        }
    }

    pub fn len(&self) -> usize {
        let n = &self.list.len();
        return *n
    }

    pub fn push(&mut self, _element: T) {
        self.list.push(_element);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.list.last()
    }


    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        self.list.reverse();
        self
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();

        for item in iter {
            list.push(item);
        }

        list
    }
}


// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::new();

        while let Some(value) = linked_list.pop() {
            vec.push(value);
        }
        vec.reverse();

        vec
    }
}

