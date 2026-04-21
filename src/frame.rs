pub mod renderPipeline {
    use std::f64::{self, consts::PI};
    /*
    this struct might be used later.
    I want to use this for configuring the
    shapes and speed of the rendering, maybe, 
    maybe not.
     */
    pub struct RenderConfig {
        shapeType : String,
        fps : i32


    }
    impl RenderConfig {
        pub fn new(shapeType: String, fps: i32)-> Self{
            Self {
                shapeType,
                fps
            }
        }
    }
    /*
    this is where all of the rendering logic will reside.
    the frame will have a function that returns a char buffer containing 
    each character to output to the terminal
     */
    pub struct Frame {
        pub rows : usize, //height of the terminal
        pub cols : usize, // width of the terminal
        pub a : f64, //used for the starting rotation of the torus
        pub b : f64, //same deal here
        pub buffer : Vec<char>
    }
    impl Frame {
        // constructor
        pub fn new(rows: i64, cols: i64, a: f64, b: f64)-> Self{
            Frame {
                rows : rows as usize,
                cols :cols as usize,
                a,
                b,
                buffer : vec![' '; rows as usize * cols as usize] 

            }
        }
        fn idx(&self, col : i32, row : i32) -> usize{
            return (col + row * self.cols as i32) as usize;
        }
        /*
        the rendering function that renders and returns the char buffer
         */
        pub fn renderFrame(&mut self, a : f64, b : f64){
            let theta_spacing = 0.07; //controls how precise the torus is around the tube
            let phi_spacing = 0.02; // controls how precise the torus is around the donut
            let r1 : f64 = 1.0; //legit no idea what these do
            let r2 : f64 = 1.7;
            let k2 : f64 = 40.0;
            let zoom = 0.3;
            let k1 = zoom * (self.cols as f64 * k2 * 3.0 / (8.0 * (r1 + r2))); // this is doing something

            let mut zbuffer = vec![0.0; self.rows * self.cols]; // holds the z position for our torus

            /*
            this my friend is the retarded work around I had to do
            since Rust is too cool to allow us to use normal for loops


            essentially this is drawing the circle, the outer loop
            draws around the tube of the torus and the inner loop draws around the donut
            and does all the other calculations
             */
            let mut theta: f64 = 0.0;
            let mut phi : f64 = 0.0;
            while theta < 2 as f64*PI {
                phi = 0.0;
                while phi < 2 as f64*PI {

                    // x,y coords of the circle

                    let circlex : f64 = r2 + r1 * theta.cos();
                    let circley : f64 = r1 * theta.sin();
          
                    //3d coords post rotation
                    let x : f64 = circlex*(b.cos()*phi.cos() + a.sin()*b.sin()*phi.sin())
                                    - circley * a.cos() * b.sin();
                    let y : f64 = circlex*(b.sin()*phi.cos() - a.sin()*b.cos()*phi.sin())
                                    + circley * a.cos() * b.cos();

                    let z : f64 = k2 + a.cos()* circlex * phi.sin() + circley * a.sin();

                    let ooz : f64 = 1.0/z;

                    let xp : i32 = ((self.cols as f64 / 2.0) + k1 * ooz * x) as i32;

                    let yp : i32 = ((self.rows as f64 / 2.0) - k1 * ooz * y) as i32;

                    let l : f64 = phi.cos()*theta.cos()*b.sin() - a.cos()*theta.cos()*phi.sin() - a.sin()*theta.sin() + b.cos()
                                 *(a.cos()*theta.sin() - theta.cos()*a.sin()* phi.sin());

                    if l > 0.0 && self.idx(xp, yp) <= zbuffer.len(){
                        if ooz > zbuffer[self.idx(xp, yp)] {
                            let idx = self.idx(xp, yp);
                            zbuffer[idx] = ooz;
                            let lum_idx : i32 = (l*8.0) as i32;
                            
                            self.buffer[idx] = ".,-~:;=!*#$@"
                                                            .chars()
                                                            .nth(lum_idx as usize)
                                                            .expect("I have no idea how this wasnt a char");
                            
                            
                        }else if self.idx(xp, yp) >= zbuffer.len(){
                            eprintln!("OOB: xp={xp} yp={yp} rows={} cols={} x={x:.2} y={y:.2} z={z:.2} ooz={ooz:.3}", self.rows, self.cols);
                        }
                    }
                    phi += phi_spacing;
                }
                theta += theta_spacing;
            }
        }
    }
}