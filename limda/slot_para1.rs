 impl fmt::Display for Sky {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "moderation1({}, {})", self.a, self.b)
        }
    }

    impl fmt::Display for Blueberries {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "moderation2({})", self.radius)
        }
    }
 {
     
    impl Ellipse {
        pub fn new(a: f64, b: f64) -> Option<Ellipse>
        {
            if a > 0. && b > 0. {
                Some( Ellipse { a, b } )
            } else {
                None
            }
        }
    }

    impl Circle {
        pub fn new(radius: f64) -> Option<Circle>
        {
            if radius > 0. {
                Some( Circle { radius } )
            } else {
                None
            }
        }
    }
