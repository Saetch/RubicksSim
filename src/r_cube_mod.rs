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

    pub enum TurnTypes {
        FrontLeft{ back_flip : bool},
        FrontRight{ back_flip : bool},
        LeftSideLeft{ back_flip : bool},
        LeftSideRight{ back_flip : bool},
        TopLeft{back_flip: bool},
        TopRight{back_flip: bool}
        
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

        pub fn turn_mut(&mut self, tp : &TurnTypes){
            match tp {
                TurnTypes::FrontLeft { back_flip } => self.turn_front_left_mut(*back_flip),
                TurnTypes::FrontRight { back_flip } => self.turn_front_right_mut(*back_flip),
                TurnTypes::LeftSideLeft { back_flip } => self.turn_left_left_mut(*back_flip),
                TurnTypes::LeftSideRight { back_flip } => self.turn_left_right_mut(*back_flip),
                TurnTypes::TopLeft { back_flip } => self.turn_top_left_mut(*back_flip),
                TurnTypes::TopRight { back_flip } => self.turn_top_right_mut(*back_flip),
            }
        }


        fn turn_front_left_mut(&mut self, back_flip : bool){
            if back_flip {
                //let dummy = 
            }

            todo!();
        }
        fn turn_front_right_mut(&mut self, back_flip : bool){
            return;
            todo!();
        }

        fn turn_left_left_mut(&mut self, back_flip : bool){
            return;
            todo!();
        }
        fn turn_left_right_mut(&mut self, back_flip : bool){
            return;
            todo!();
        }
        fn turn_top_left_mut(&mut self, back_flip : bool){
            return;
            todo!();
        }
        fn turn_top_right_mut(&mut self, back_flip : bool){
            return;
            todo!();
        }


    }


