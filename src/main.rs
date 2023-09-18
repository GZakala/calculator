fn main() {
    let num1 = CalcLineNumber::new(25 as f64);
    let num2 = CalcLineNumber::new(10 as f64);

    let op1 = 
        CalcLineOperation::new(MathOperations::Plus)
        .set_left(&num1)
        .set_right(&num2);

}

#[derive(Debug)]
enum CalcLineNode<'a> {
    Num(CalcLineNumber<'a>),
    Op(CalcLineOperation<'a>),
}

impl<'a> CalcLineNode<'a> {
    fn new(num: f64) -> Self {
        CalcLineNode::Num(
            CalcLineNumber::new(num)
        )
    }

    fn get_left(&self) -> 
}

impl<'a> ToString for CalcLineNode<'a> {
    fn to_string(&self) -> String {
        let node = self;
        let mut res = String::new();
        loop {
            match node {
                CalcLineNode::Num(calc_num) => {
                    res.push_str(format!(" {}", calc_num.num).as_str())
                    
                },
                CalcLineNode::Op(calc_op) => {
                    res.push_str(format!(" {}", calc_op.op).as_str())
                },
                None => break
            }
        }
        res
    }
}

#[derive(Debug)]
struct CalcLineOperation<'a> {
    op: MathOperations,
    left_num: Option<&'a CalcLineNode<'a>>,
    right_num: Option<&'a CalcLineNode<'a>>,
}

#[derive(Debug)]
enum MathOperations {
    Plus,
    Minus,
    Mult,
    Div,
}

impl<'a> CalcLineOperation<'a> {
    fn new(op: MathOperations) -> Self {
        Self { op, left_num: None, right_num: None }
    }

    fn set_left(mut self, left_number: &'a CalcLineNumber) -> Self {
        self.left_num = Some(left_number);
        self
    }

    fn set_right(mut self, right_number: &'a CalcLineNumber) -> Self {
        self.left_num = Some(right_number);
        self
    }

    fn plus(self) -> Result<CalcLineNumber<'a>, &'static str> {
        match (self.left_num, self.right_num) {
            (Some(left_num), Some(right_num)) => {
                let num = left_num.num + right_num.num;
                let mut calc_line_num = CalcLineNumber::new(num);
                if let Some(op) = left_num.left_operation {
                    calc_line_num = calc_line_num.set_left(op);
                }
                if let Some(op) = right_num.right_operation {
                    calc_line_num = calc_line_num.set_right(op);
                }
                Ok(calc_line_num)
            },
            (Some(left_num), None) => {
                let num = left_num.num;
                let mut calc_line_num = CalcLineNumber::new(num);
                if let Some(op) = left_num.left_operation {
                    calc_line_num = calc_line_num.set_left(op);
                }
                Ok(calc_line_num)
            },
            (None, Some(right_num)) => {
                let num = right_num.num;
                let mut calc_line_num = CalcLineNumber::new(num);
                if let Some(op) = right_num.right_operation {
                    calc_line_num = calc_line_num.set_right(op);
                }
                Ok(calc_line_num)
            },
            (None, None) => Err("Попытка проссумировать отсутствующие значения!")
        }
    }

}

#[derive(Debug)]
struct CalcLineNumber<'a> {
    num: f64,
    left_operation: Option<&'a CalcLineNode<'a>>,
    right_operation: Option<&'a CalcLineNode<'a>>,
}

impl<'a> CalcLineNumber<'a> {
    fn new(num: f64) -> Self {
        Self { num, left_operation: None, right_operation: None }
    }

    fn set_left(mut self, left: &'a CalcLineNode) -> Self {
        self.left_operation = Some(left);
        self
    }

    fn set_right(mut self, right: &'a CalcLineNode) -> Self {
        self.right_operation = Some(right);
        self
    }
}



// use iced::widget::button;
// use iced::{Alignment, Element, Sandbox, Settings};


// pub fn main() -> iced::Result {
//     Calculator::run(Settings::default())
// }

// struct Calculator {
//     line: String,
//     value: f64,
// }

// #[derive(Debug, Clone, Copy)]
// enum Message {
//     TabOne,
//     TabTwo,
//     TabThree,
//     TabFour,
//     TabFive,
//     TabSix,
//     TabSeven,
//     TabEight,
//     TabNine,
//     Clean
// }

// impl Sandbox for Calculator {
//     type Message = Message;

//     fn new() -> Self {
//         Self { value: 0 as f64 }
//     }
    
//     fn title(&self) -> String {
//         String::from("Calculator")
//     }

//     fn update(&mut self, message: Message) {
//         use Message::*;
//         match message {
//             TabOne => self.
//         }
//     }
// }

// struct CalculatorLine {
//     numbers: [f64],
// }

// struct CalculatorLineNumber {
//     num: i64,
//     operation: MathOperations,
// }

// enum MathOperations {
//     Add
// }
