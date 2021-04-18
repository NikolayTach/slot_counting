 impl fmt::Display for Ellipse {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "эллипс({}, {})", self.a, self.b)
        }
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "круг({})", self.radius)
        }
    }

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
