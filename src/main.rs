
use std::ops;

struct Value {
    data: f32,
    grad: f32
}

enum Node {
    Load(Box<Value>),
    Add(Box<(Node, Node)>),
    Sub(Box<(Node, Node)>),
    Mul(Box<(Node, Node)>),
    Div(Box<(Node, Node)>)
}



impl ops::Add for Node {
    type Output = Node;

    fn add(self, rhs: Self) -> Self::Output {
        Node::Add(Box::new((self, rhs)))
    }
}

impl ops::Sub for Node {
    type Output = Node;

    fn sub(self, rhs: Self) -> Self::Output {
        Node::Sub(Box::new((self, rhs)))
    }
}

impl ops::Mul for Node {
    type Output = Node;

    fn mul(self, rhs: Self) -> Self::Output {
        Node::Mul(Box::new((self, rhs)))
    }
}
impl ops::Div for Node {
    type Output = Node;

    fn div(self, rhs: Self) -> Self::Output {
        Node::Div(Box::new((self, rhs)))
    }
}

impl Node {
    fn new(data: f32, grad: f32) -> Node {
        Node::Load(Box::new(Value { data, grad }))
    }

    fn resolve(self) -> Value {
        use Node::*;
        match self {
            Load(val) => *val,
            Add(vals) => {
                let lhs = (*vals).0.resolve();
                let rhs = (*vals).1.resolve();
                Value {
                    data: lhs.data + rhs.data,
                    grad: lhs.grad + rhs.grad
                }
            },
            Sub(vals) => {
                let lhs = (*vals).0.resolve();
                let rhs = (*vals).1.resolve();

                Value {
                    data: lhs.data - rhs.data,
                    grad: lhs.grad - rhs.grad
                }
            },
            Mul(vals) => {
                let lhs = (*vals).0.resolve();
                let rhs = (*vals).1.resolve();
                Value {
                    data: lhs.data * rhs.data,
                    grad: lhs.data*rhs.grad + lhs.grad*rhs.data
                }
            },
            Div(vals) => {
                let lhs = (*vals).0.resolve();
                let rhs = (*vals).1.resolve();

                assert_ne!(rhs.data, 0.0);
                Value {
                    data: lhs.data / rhs.data,
                    grad: (lhs.grad*rhs.data - lhs.data*rhs.grad) / (rhs.data*rhs.data)
                }
            }
        }
    }

}




fn main() {

    let a = Node::new(1.0, 2.0);
    let b = Node::new(-2.0, 2.0);
    let c = Node::new(0.0, -1.0);


    let ans = (a + b) * c;

    let a_d = 1.0f32;
    let a_g = 2.0f32;
    let b_d = -2.0f32;
    let b_g = 2.0f32;
    let c_d = 0.0f32;
    let c_g = -1.0f32;

    let ans_d = (a_d + b_d) * c_d;
    let ans_g = (a_g + b_g) * c_d + (a_d + b_d) * c_g;


    let result = ans.resolve();

    assert_eq!(result.data, ans_d);
    assert_eq!(result.grad, ans_g);


}
