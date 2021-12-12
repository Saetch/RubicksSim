use core::fmt;
use std::ops::Add;

use ansi_term::Colour::*;


    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum RColor {
        White,
        Yellow,
        Red,
        Blue,
        Orange,
        Green
    }

    impl fmt::Display for RColor{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
            match self{
                RColor::White => Ok(write!(f, "{}", "RColor::White")?),
                RColor::Yellow => Ok(write!(f, "{}", "RColor::Yellow")?),
                RColor::Red => Ok(write!(f, "{}", "RColor::Red")?),
                RColor::Blue => Ok(write!(f, "{}", "RColor::Blue")?),
                RColor::Orange => Ok(write!(f, "{}", "RColor::Orange")?),
                RColor::Green => Ok(write!(f, "{}", "RColor::Green")?),
            }
        }
    }

    





    pub struct CubeSide{

       stones : [RColor; 9]


    }



    impl CubeSide{
        pub fn new(input: RColor) -> Self{
            let mut ret:CubeSide = CubeSide{
                stones : [RColor::White;9]
            };
            for i in 0..=8{
                ret.stones[i] = input;
            }
            return ret;

        }

        pub fn is_solved(&self)-> bool{
            let cmp = self.stones[0];
            for st in self.stones{
                if !(st == cmp){
                    return false;
                }
            }
            true
        }
    

        pub fn stringify(&self) -> String{
           
                
            let mut ret_string = String::new();
            let mut counter = 0;
            for st in self.stones{
                ret_string = ret_string.add(&color_stone(&st));
                counter +=1;
                if counter % 3 == 0 {
                    ret_string = ret_string.add("\n");
                }
            }

            return ret_string;
            
        }

        

        

        pub fn set_left_mut(&mut self, cl : &[RColor;3]) ->[RColor;3]{
            let mut retete = [RColor::Blue; 3];
            for i in 0..3{
                retete[i] = self.stones[i*3];
                self.stones[i*3] = cl[i];
            }            
            return retete;
        }

        

        pub fn set_right_mut(&mut self, cl : &[RColor;3]) -> [RColor;3]{
            let mut retete = [RColor::Blue; 3];

            for i in 0..3{
                retete[i] = self.stones[2+i*3];
                self.stones[2+i*3] = cl[i];
            }      
            return retete;      
        }

    
        pub fn get_left(&self) -> [RColor;3]{
            let mut retete = [RColor::Blue; 3];
            for i in 0..3{
                retete[i] = self.stones[i*3];
            } 
            return retete;
        }

        pub fn get_right(&self) -> [RColor;3]{
            let mut retete = [RColor::Blue; 3];
            for i in 0..3{
                retete[i] = self.stones[2+i*3];
            } 
            return retete;
        }

        pub fn set_top_mut(&mut self, cl :&[RColor; 3]) -> [RColor;3]{
            let mut retete = [RColor::Blue;3];
            for i in 0..cl.len(){
                retete[i] = self.stones[i];
                self.stones[i] = cl[i];
            }
            return retete;
        }


        pub fn set_bot_mut(&mut self, cl : &[RColor; 3]) -> [RColor;3]{
            let mut retete = [RColor::Blue;3];
            for i in 0..cl.len(){
                retete[i] = self.stones[6+i];
                self.stones[6+i] = cl[i];
            }
            return retete;
        }



        pub fn get_top(&self)-> [RColor;3]{
            let mut retete = [RColor::Blue;3];
            for i in 0..retete.len(){
                retete[i] = self.stones[i];
            }
            return retete;
        }

        pub fn get_bot(&self)-> [RColor;3]{
            let mut retete = [RColor::Blue;3];
            for i in 0..retete.len(){
                retete[i] = self.stones[6+i];
            }
            return retete;
        }


        pub fn rotate_left(&mut self){
            //start with corners
            let mut dum = self.stones[2];
            self.stones[2] = self.stones[8];
            self.stones[8] = self.stones[6];
            self.stones[6] = self.stones[0];
            self.stones[0] = dum;

            dum = self.stones[3];
            self.stones[3] = self.stones[1];
            self.stones[1] = self.stones[5];
            self.stones[5] = self.stones[7];
            self.stones[7] = dum;
        }

        pub fn rotate_right(&mut self){
             //start with corners
             let dum = self.stones[2];
             self.stones[2] = self.stones[0];
             self.stones[0] = self.stones[6];
             self.stones[6] = self.stones[8];
             self.stones[8] = dum;

             //continue with the other ones
             let dum = self.stones[5];
             self.stones[5] = self.stones[1];
             self.stones[1] = self.stones[3];
             self.stones[3] = self.stones[7];
             self.stones[7] = dum;
        }

        pub fn get_Solved(&self) -> Vec<u8>{
            let mut retete : Vec<u8> = Vec::new();
            let _to_check = self.stones[4];
            for i  in 0..=8{
                match retete[i] {
                    _to_check => retete.push(i.try_into().unwrap()),
                    _ => ()
                }
            }
            return retete.iter().filter(|x| *x == (4.try_into().unwrap())).cloned().collect();
        }
    }

    fn color_stone( i : &RColor) -> String{
        match i{
            RColor::White => Black.on(White).paint("[ ]").to_string(),
            RColor::Yellow => Black.on(Yellow).paint("[ ]").to_string(),
            RColor::Red => Black.on(Red).paint("[ ]").to_string(),
            RColor::Blue => Black.on(Blue).paint("[ ]").to_string(),
            RColor::Orange => Black.on(RGB(0xFF, 0x88, 0x00)).paint("[ ]").to_string(),
            RColor::Green => Black.on(Green).paint("[ ]").to_string(),
        }
    }

