// todo: question_mark
/* Instructions
Create the following structures. Each has one element:

One: first_layer as type Option<Two>.
Two: second_layer as type Option<Three>
Three: third_layer as type Option<Four>
Four: fourth_layer as type Option<u16>.
Beside the structures, you must create a function named get_fourth_layer, and associate it to the One structure. This function should return the Option value in the Four structure.

Expected Function and structures
pub struct One {
    // expected public fields
}
pub struct Two {
    // expected public fields
}
pub struct Three {
    // expected public fields
}
pub struct Four {
    // expected public fields
}

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {}
} */

pub struct One {
    pub first_layer: Option<Two>,
}

pub struct Two {
    pub second_layer: Option<Three>,
}

pub struct Three {
    pub third_layer: Option<Four>,
}

pub struct Four {
    pub fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {
        self.first_layer.as_ref().and_then(|two| {
            two.second_layer.as_ref().and_then(|three| {
                three.third_layer.as_ref().and_then(|four| {
                    four.fourth_layer.as_ref().and_then(|fourth_layer| Some(*fourth_layer))
                })
            })
        })
    }
}