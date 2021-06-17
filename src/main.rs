use ndarray::prelude::*;

fn main() {
  // test closure to get ndim
  let getdim = |p_arr: &Array<f32, _>| {
    p_arr.ndim()
  };

  // return squared input array
  // can't handle differnt array ndims.
  // all arrays must have same ndim during execution
  // needs change                                                      I think
  let square = |p_arr: &Array<f32, _>| {
    p_arr.mapv(|p_arr: f32| p_arr.powi(2))
  };

  //let maximum = |p_arr: &Array<f32, _>| {
  //  // needs to be implemented
  //};

  //let leakyRelu = |p_arr: &Array<f32, _>| {
  //  // needs to be implemented
  //};

  // testing some stuff

  let x = array![1.,2.,3.];
  let y = array![4.,5.,6.];

  println!("The ndarray x: {}", x.view());
  println!("The ndarray y: {}", y.view());

  println!("X + Y: {}", &x+&y);
  println!("X * Y: {}", &x*&y);
  println!("X dot Y: {}", &x.dot(&y));
  
  // test square
  //println!("square x: {}", square(&x));
  // test max
//  println!("maximum x y: {}", maximum(&x,&y));
  // test getdim
  println!("getdim x: {}", getdim(&x));


  println!{"\n"};

  let b = array![
    [[1.,2.],[3.,4.]]
  ];

  println!("The ndarray b:\n {}", b.view());

  println!("sum b axis 0:\n {}", &b.sum_axis(Axis(0))); // seems like axis 0 is just the array itself
  println!("sum b axis 1:\n {}", &b.sum_axis(Axis(1)));
  println!("sum b axis 2:\n {}", &b.sum_axis(Axis(2)));

  println!{"\n"};

  // add 1D array to last axis
  let c = array![[1.,2.,3.,],[4.,5.,6.]];
  let d = array![10.,20.,30.];

  println!("c + d: {}", c+d);
  
  println!{"\n"};

  let a = array![
    [1.,2.,3.],
    [4.,5.,6.],
  ];

  // test square
  println!("square a: {}", square(&a));
 
  assert_eq!(a.ndim(), 2);
  assert_eq!(a.len(), 6);
  assert_eq!(a.shape(), [2, 3]);
  assert_eq!(a.is_empty(), false);

  println!("{:?}", a);


}
