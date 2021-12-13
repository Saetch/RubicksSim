use std::ops::Add;

use crate::cube_side_mod::CubeSide as CubeSide;
use crate::cube_side_mod::RColor as RColor;
    //for a rubik's Cube current state
    #[derive(Clone, Copy)]
    pub struct RCube{
        top : CubeSide,
        front : CubeSide,
        left : CubeSide,
        back : CubeSide,
        right : CubeSide,
        bottom : CubeSide
    }

    #[derive(Hash, std::cmp::Eq, PartialEq, Clone, Copy)]
    pub enum TurnTypes {
        FrontLeft{ back_flip : bool},
        FrontRight{ back_flip : bool},
        LeftSideLeft{ back_flip : bool},
        LeftSideRight{ back_flip : bool},
        TopPlatform{back_flip: bool},
        BottomPlatform{back_flip: bool}
        
    }
#[macro_export]
    macro_rules! u {
        () => {
            TurnTypes::TopPlatform{back_flip:true}
        };
    }
    #[macro_export]

    macro_rules! up {
        () => {
            TurnTypes::TopPlatform{back_flip:false}
        };
    }
    #[macro_export]

    macro_rules! d {
        () => {
            TurnTypes::BottomPlatform{back_flip:false}
        };
    }
    #[macro_export]

    macro_rules! dp {
        () => {
            TurnTypes::BottomPlatform{back_flip:true}

        };
    }
    #[macro_export]

    macro_rules! f {
        () => {
            TurnTypes::LeftSideRight{ back_flip : true}
        };
    }
    #[macro_export]

    macro_rules! fp {
        () => {
            TurnTypes::LeftSideRight{ back_flip : false}
        };
    }
    #[macro_export]

    macro_rules! b {
        () => {
            TurnTypes::LeftSideLeft{ back_flip : false}
        };
    }
    #[macro_export]

    macro_rules! bp {
        () => {
            TurnTypes::LeftSideLeft{ back_flip : true}
        }
    }
    #[macro_export]

    macro_rules! l {
        () => {
            TurnTypes::FrontLeft{ back_flip:false}
        };
    }
    #[macro_export]

    macro_rules! lp {
        () => {
            TurnTypes::FrontLeft{ back_flip:true}
        };
    }
    #[macro_export]

    macro_rules!  r{
        () => {
            TurnTypes::FrontRight{back_flip:true}
        };
    }
    #[macro_export]

    macro_rules! rp {
        () => {
            TurnTypes::FrontRight{back_flip:false}
        };
    }

    macro_rules! top {
        ($se:expr,$x:expr) => {
            $se.top.stones[$x] == white!() 

            
        };
    }
    macro_rules! front {
        ($se:expr,$x:expr) => {            $se.front.stones[$x] == green!()
        };
    }
    macro_rules! left {
        ($se:expr,$x:expr) => {            $se.left.stones[$x] == orange!()
        };
    }
    macro_rules! right{
        ($se:expr,$x:expr) => {            $se.right.stones[$x] == red!()
        };
    }
    macro_rules! back {
        ($se:expr,$x:expr) => {            $se.back.stones[$x] == blue!()
        };
    }
    macro_rules! bot {
        ($se:expr,$x:expr) => {            $se.bottom.stones[$x] == yellow!()
        };
    }

    macro_rules! white {
        () => {
            RColor::White
        };
    }
    macro_rules! red {
        () => {
            RColor::Red
        };
    }
    macro_rules! green {
        () => {
            RColor::Green
        };
    }
    macro_rules! orange {
        () => {
            RColor::Orange
        };
    }
    macro_rules! blue {
        () => {
            RColor::Blue
        };
    }
    macro_rules! yellow {
        () => {
            RColor::Yellow
        };
    }

    #[macro_export]
    macro_rules! check {
        (  $x:expr, $xi:expr, $xcolor:expr, , $y:expr, $yi:expr, $ycolor:expr,$z:expr, $zi:expr, $zcolor:expr  ) => {
            {
              $x.stones[$xi] == $xcolor && $y.stones[$yi] == $ycolor && $z.stones[$zi] == $zcolor
                
            }
        };
    }
    impl Eq for RCube{
        
    }


    impl PartialEq for RCube{
        fn eq(&self, other: &Self) -> bool {
        for i in 0..9{
            if self.top.stones[i] != other.top.stones[i]{
                return false;
            }
        }
        for i in 0..9{
            if self.front.stones[i] != other.front.stones[i]{
                return false;
            }
        }
        for i in 0..9{
            if self.left.stones[i] != other.left.stones[i]{
                return false;
            }
        }
        for i in 0..9{
            if self.right.stones[i] != other.right.stones[i]{
                return false;
            }
        }
        for i in 0..9{
            if self.back.stones[i] != other.back.stones[i]{
                return false;
            }
        }
        for i in 0..9{
            if self.bottom.stones[i] != other.bottom.stones[i]{
                return false;
            }
        }



        return true;
    }
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

        pub fn turn_mut_ret(&mut self, tp : &TurnTypes)->Self{
            match tp {
                TurnTypes::FrontLeft { back_flip } => self.turn_front_left_mut(*back_flip),
                TurnTypes::FrontRight { back_flip } => self.turn_front_right_mut(*back_flip),
                TurnTypes::LeftSideLeft { back_flip } => self.turn_left_left_mut(*back_flip),
                TurnTypes::LeftSideRight { back_flip } => self.turn_left_right_mut(*back_flip),
                TurnTypes::TopPlatform { back_flip } => self.turn_top_platform_mut(*back_flip),
                TurnTypes::BottomPlatform { back_flip } => self.turn_bottom_platform_mut(*back_flip),
            }
            return *self;
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

        pub fn _get_solved_amount(&self)-> u8{
            let mut _count : u8 = 0 as u8;
            

            let _top_solved = self.top._get_solved();
            let _front_solved = self.front._get_solved();
            let _left_solved = self.left._get_solved();
            let _right_solved = self.right._get_solved();
            let _bottom_solved = self.bottom._get_solved();
            let _back_solved = self.back._get_solved();
            todo!();
        }

        pub fn hypo_move(&self, turn : TurnTypes) -> Self{
            let mut retete = self.clone();
            retete.turn_mut(&turn);
            return retete;
        }

        pub fn get_solved_amount_top_first(&self)-> u16{
            //the 6 middle stones are always solved and don't need to be checked
            //this could be 0 and this is just implicitly assumed, but expressing it seems more intuitive
            let mut count:u16 = 60;
            //TOP PLATFORM
            if    self.top.stones[6] == RColor::White && self.front.stones[0] == RColor::Green && self.left.stones[2]==RColor::Orange{
                count += 100;
            }
            if self.top.stones[7] == RColor::White && self.front.stones[1] == RColor::Green{
                count += 100; 
            }
            if top!(self,8) && front!(self,2)  && right!(self,0){
                count +=100;
            }
            if top!(self, 3) && left!(self, 1){
                count +=100;
            }
            if top!(self, 5) && right!(self, 1) {
                count +=100;

            }
            if top!(self,0) && left!(self,0)  && back!(self,6){
                count +=100;
            }
            if top!(self,1)  && back!(self,7) {
                count +=100;

            }
            if top!(self,2)  && back!(self,8)  && right!(self, 2) {
                count +=100;
            }

            //Middle "PLATFORM"
            if front!(self,3) && left!(self,5){
                count +=10;
            }
            if front!(self,5) && right!(self,3){
                count +=10;
            }
            if right!(self,5) && back!(self, 5){
                count +=10;
            }
            if back!(self, 3) && left!(self, 3){
                count +=10;
            }


            //Bottom Platform
            if front!(self, 6) && left!(self, 8) && bot!(self,0){
                count +=1;
            }
            if front!(self, 7) && bot!(self, 1){
                count +=1;
            }
            if front!(self, 8) && bot!(self, 2) && right!(self, 6){
                count +=1;
            }
            if right!(self, 7) && bot!(self, 5){
                count +=1;
            }
            if right!(self, 8) && bot!(self, 8) && back!(self, 2){
                count +=1;
            }
            if bot!(self, 7) && back!(self, 1){
                count +=1;
            } 
            if bot!(self, 6) && back!(self, 0) && left!(self, 6){
                count +=1;
            }
            if left!(self, 7) && bot!(self, 4){
                count +=1;
            }


            //In order for the basic algorithm to work, it cannot focus on keeping 8/9 stones on top correct, as the moves that would result in 9/9 would be the
            //last ones to be tried

            count
        }

        pub fn get_solved_amount_direct(&self)-> u8{
            //the 6 middle stones are always solved and don't need to be checked
            //this could be 0 and this is just implicitly assumed, but expressing it seems more intuitive
            let mut count:u8 = 6;
            //TOP PLATFORM
            if    self.top.stones[6] == RColor::White && self.front.stones[0] == RColor::Green && self.left.stones[2]==RColor::Orange{
                count += 1;
            }
            if self.top.stones[7] == RColor::White && self.front.stones[1] == RColor::Green{
                count += 1; 
            }
            if top!(self,8) && front!(self,2)  && right!(self,0){
                count +=1;
            }
            if top!(self, 3) && left!(self, 1){
                count +=1;
            }
            if top!(self, 5) && right!(self, 1) {
                count +=1;

            }
            if top!(self,0) && left!(self,0)  && back!(self,6){
                count +=1;
            }
            if top!(self,1)  && back!(self,7) {
                count +=1;

            }
            if top!(self,2)  && back!(self,8)  && right!(self, 2) {
                count +=1;
            }

            //Middle "PLATFORM"
            if front!(self,3) && left!(self,5){
                count +=1;
            }
            if front!(self,5) && right!(self,3){
                count +=1;
            }
            if right!(self,5) && back!(self, 5){
                count +=1;
            }
            if back!(self, 3) && left!(self, 3){
                count +=1;
            }

            


            //Bottom Platform
            if front!(self, 6) && left!(self, 8) && bot!(self,0){
                count +=1;
            }
            if front!(self, 7) && bot!(self, 1){
                count +=1;
            }
            if front!(self, 8) && bot!(self, 2) && right!(self, 6){
                count +=1;
            }
            if right!(self, 7) && bot!(self, 5){
                count +=1;
            }
            if right!(self, 8) && bot!(self, 8) && back!(self, 2){
                count +=1;
            }
            if bot!(self, 7) && back!(self, 1){
                count +=1;
            } 
            if bot!(self, 6) && back!(self, 0) && left!(self, 6){
                count +=1;
            }
            if left!(self, 7) && bot!(self, 4){
                count +=1;
            }



            return count;
        }


        pub fn clone(&self) -> Self{
            let mut ret = RCube::new();

                ret.top.stones = self.top.stones.clone();
                ret.front.stones = self.front.stones.clone();
                ret.left.stones = self.left.stones.clone();
                ret.right.stones = self.right.stones.clone();
                ret.back.stones = self.back.stones.clone();
                ret.bottom.stones = self.bottom.stones.clone();

                ret
        
        }
        

    }


    fn turn_array( arr : &[RColor; 3]) -> [RColor;3]{
        let mut retete = arr.clone();
        retete[0] = arr[2];
        retete[2] = arr[0];
        return retete;
    }


