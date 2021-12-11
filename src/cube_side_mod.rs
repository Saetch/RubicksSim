use core::fmt;
use std::any::Any;

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
                RColor::White => write!(f, "{}", "RColor::White"),
                RColor::Yellow => write!(f, "{}", "RColor::Yellow"),
                RColor::Red => write!(f, "{}", "RColor::Red"),
                RColor::Blue => write!(f, "{}", "RColor::Blue"),
                RColor::Orange => write!(f, "{}", "RColor::Orange"),
                RColor::Green => write!(f, "{}", "RColor::Green"),
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
    }

