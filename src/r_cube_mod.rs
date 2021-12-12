use std::ops::Add;

use crate::cube_side_mod::CubeSide as CubeSide;
use crate::cube_side_mod::RColor as RColor;
    //for a rubik's Cube current state
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
        TopPlatform{back_flip: bool},
        BottomPlatform{back_flip: bool}
        
    }

    impl TurnTypes{
        pub fn switch_direction(&self) -> TurnTypes{
            match self {
                TurnTypes::FrontLeft { back_flip } => TurnTypes::FrontLeft { back_flip : !*back_flip },
                TurnTypes::FrontRight { back_flip } => TurnTypes::FrontRight { back_flip : !*back_flip },
                TurnTypes::LeftSideLeft { back_flip } => TurnTypes::LeftSideLeft { back_flip : !*back_flip },
                TurnTypes::LeftSideRight { back_flip } => TurnTypes::LeftSideRight { back_flip: !*back_flip  },
                TurnTypes::TopPlatform { back_flip } => TurnTypes::TopPlatform { back_flip: !*back_flip  },
                TurnTypes::BottomPlatform { back_flip } => TurnTypes::BottomPlatform { back_flip: !*back_flip  },
            }
        }
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
                    front : CubeSide::new(RColor::Green),
                    left : CubeSide::new(RColor::Orange),
                    back : CubeSide::new(RColor::Blue),
                    right : CubeSide::new(RColor::Red),
                    bottom : CubeSide::new(RColor::Yellow)
                
            }

        }

        pub fn turn_mut(&mut self, tp : &TurnTypes){
            match tp {
                TurnTypes::FrontLeft { back_flip } => self.turn_front_left_mut(*back_flip),
                TurnTypes::FrontRight { back_flip } => self.turn_front_right_mut(*back_flip),
                TurnTypes::LeftSideLeft { back_flip } => self.turn_left_left_mut(*back_flip),
                TurnTypes::LeftSideRight { back_flip } => self.turn_left_right_mut(*back_flip),
                TurnTypes::TopPlatform { back_flip } => self.turn_top_platform_mut(*back_flip),
                TurnTypes::BottomPlatform { back_flip } => self.turn_bottom_platform_mut(*back_flip),
            }
        }


        fn turn_front_left_mut(&mut self, back_flip : bool){
            if back_flip {
                self.front.set_left_mut(&self.bottom.set_left_mut(&self.back.set_left_mut(&self.top.set_left_mut(&self.front.get_left())))); 
                self.left.rotate_left();
            }
            else{
                self.front.set_left_mut(&self.top.set_left_mut(&self.back.set_left_mut(&self.bottom.set_left_mut(&self.front.get_left()))));
                self.left.rotate_right();
            }

            
        }
        fn turn_front_right_mut(&mut self, back_flip : bool){
            if back_flip{
                self.front.set_right_mut(&self.bottom.set_right_mut(&self.back.set_right_mut(&self.top.set_right_mut(&self.front.get_right()))));
                self.right.rotate_right();
            }else{
                self.front.set_right_mut(&self.top.set_right_mut(&self.back.set_right_mut(&self.bottom.set_right_mut(&self.front.get_right()))));
                self.right.rotate_left();
            }
        }

        fn turn_left_left_mut(&mut self, back_flip : bool){
            if back_flip{
                self.left.set_left_mut(&self.bottom.set_bot_mut(&turn_array(&self.right.set_right_mut(&self.top.set_top_mut(&turn_array(&self.left.get_left()))))));
                self.back.rotate_left();
            }else{
                self.left.set_left_mut(&turn_array(&self.top.set_top_mut(&self.right.set_right_mut(&turn_array(&self.bottom.set_bot_mut(&self.left.get_left()))))));
                self.back.rotate_right();
            }
        }
        fn turn_left_right_mut(&mut self, back_flip : bool){
            if back_flip{
                self.left.set_right_mut(&self.bottom.set_top_mut(&turn_array(&self.right.set_left_mut(&self.top.set_bot_mut(&turn_array(&self.left.get_right()))))));
                self.front.rotate_right();
            }else{
                self.left.set_right_mut(&turn_array(&self.top.set_bot_mut(&self.right.set_left_mut(&turn_array(&self.bottom.set_top_mut(&self.left.get_right()))))));
                self.front.rotate_left();
            }
        }
        fn turn_top_platform_mut(&mut self, back_flip : bool){
            if back_flip{
                self.front.set_top_mut(&self.right.set_top_mut(&turn_array(&self.back.set_bot_mut(&turn_array(&self.left.set_top_mut(&self.front.get_top()))))));
                self.top.rotate_right();
            }else{
                self.front.set_top_mut(&self.left.set_top_mut(&turn_array(&self.back.set_bot_mut(&turn_array(&self.right.set_top_mut(&self.front.get_top()))))));
                self.top.rotate_left();
            }
        }
        fn turn_bottom_platform_mut(&mut self, back_flip : bool){
            if back_flip{
                self.front.set_bot_mut(&self.right.set_bot_mut(&turn_array(&self.back.set_top_mut(&turn_array(&self.left.set_bot_mut(&self.front.get_bot()))))));
                self.bottom.rotate_left();
            }else{
                self.front.set_bot_mut(&self.left.set_bot_mut(&turn_array(&self.back.set_top_mut(&turn_array(&self.right.set_bot_mut(&self.front.get_bot()))))));
                self.bottom.rotate_right();
            }
        }

        pub fn is_solved(&self)->bool{
            self.front.is_solved() && self.left.is_solved() && self.back.is_solved() && self.right.is_solved() && self.top.is_solved() &&  self.bottom.is_solved()
        }


    }

    fn turn_array( arr : &[RColor; 3]) -> [RColor;3]{
        let mut retete = arr.clone();
        retete[0] = arr[2];
        retete[2] = arr[0];
        return retete;
    }


