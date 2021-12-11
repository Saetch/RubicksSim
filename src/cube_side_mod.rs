use core::fmt;
use std::ops::Add;

    #[derive(Copy, Clone)]
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

                println!("Coloring stone {}{}", &i, input);
            }
            return ret;

        }
    

        pub fn stringify(&self) -> String{
           
                
            let mut ret_string = String::new();
            let mut counter = 0;
            for st in self.stones{
                ret_string = ret_string.add(counter.to_string().add(" : ").as_str());
                counter +=1;
                ret_string = ret_string.add(st.to_string().add("\n").as_str());
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
        pub fn force_left_mut(&mut self, cl : [RColor;3]){
            for i in 0..3{
                self.stones[i*3] = cl[i];
            }            
        }
        

        pub fn set_right_mut(&mut self, cl : &[RColor;3]) -> [RColor;3]{
            let mut retete = [RColor::Blue; 3];

            for i in 0..3{
                retete[i] = self.stones[2+i*3];
                self.stones[2+i*3] = cl[i];
            }      
            return retete;      
        }
        pub fn force_right_mut(&mut self, cl : [RColor;3]){
            for i in 0..3{
                self.stones[2+i*3] = cl[i];
            }            
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

        pub fn set_top_mut(&mut self, cl :[RColor; 3]) -> [RColor;3]{
            let mut retete = [RColor::Blue;3];
            for i in 0..cl.len(){
                retete[i] = self.stones[i];
                self.stones[i] = cl[i];
            }
            return retete;
        }

        pub fn force_top_mut(&mut self, cl : [RColor;3]){
            for i in 0..3{
                self.stones[i] = cl[i];
            }            
        }

        pub fn set_bot_mut(&mut self, cl : [RColor; 3]) -> [RColor;3]{
            let mut retete = [RColor::Blue;3];
            for i in 0..cl.len(){
                retete[i] = self.stones[6+i];
                self.stones[6+1] = cl[i];
            }
            return retete;
        }

        pub fn force_bot_mut(&mut self, cl : [RColor;3]){
            for i in 0..3{
                self.stones[6+i] = cl[i];
            }            
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




    }

