pub struct triangle{
    pub a: i32,
    pub h: i32
}

pub struct rectangle{
    pub a: i32,
    pub b: i32
}


impl triangle {
    pub fn calculate_graph(&self)->i32{
        return (self.a * self.h) / 2
    }
}


impl rectangle {
    pub fn calculate_graph(&self)->i32{
        return self.a * self.b
    }
}


