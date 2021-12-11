use std::ops::Add;

use crate::cube_side_mod::CubeSide as CubeSide;
use crate::cube_side_mod::RColor as RColor;
    //for a rubick's Cube current state
    pub struct RCube{
        top : CubeSide,
        front : CubeSide,
        left : CubeSide,
        back : CubeSide,
        right : CubeSide,
        bottom : CubeSide
    }
    impl ToString for RCube{
        fn to_string(&self) -> String{
            let mut ret_string : String = String::from("");

            ret_string = ret_string.add("TOP Side\n");
            ret_string = ret_string.add(self.top.stringify().as_str());
            ret_string = ret_string.add("FRONT Side\n");
            ret_string = ret_string.add(self.front.stringify().as_str());

            ret_string = ret_string.add("LEFT Side\n");
            ret_string = ret_string.add(self.left.stringify().as_str());
            ret_string = ret_string.add("BACK Side\n");
            ret_string = ret_string.add(self.back.stringify().as_str());

            ret_string = ret_string.add("RIGHT Side\n");
            ret_string = ret_string.add(self.right.stringify().as_str());
            ret_string = ret_string.add("BOTTOM Side\n");
            ret_string = ret_string.add(self.bottom.stringify().as_str());

            return ret_string;
        }
    }

    impl RCube{
        pub fn new() -> Self{
            Self{
                
                    top : CubeSide::new(RColor::White),
                    front : CubeSide::new(RColor::Red),
                    left : CubeSide::new(RColor::Blue),
                    back : CubeSide::new(RColor::Orange),
                    right : CubeSide::new(RColor::Green),
                    bottom : CubeSide::new(RColor::Yellow)
                
            }

        }

    }


