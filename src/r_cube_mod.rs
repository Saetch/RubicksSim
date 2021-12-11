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


