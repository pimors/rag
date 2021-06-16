use ndarray::prelude::*;

// currently only squares 1d arrays
fn square(p_x: &Array1<f32>) -> Array1<f32> {
  return p_x.mapv(|p_x: f32| p_x.powi(2));
}

//fn maximum(p_x: &Array1<f32>, p_y: &Array1<f32>) -> Array1<f32> {
//  for i in p_x.iter() {
//    print!("i-1: {}\n", p_y[*i as i32])
//  }
//
//  return p_x * p_y;
//}

fn main() {
  // testing some stuff
  let a = array![
    [1.,2.,3.],
    [4.,5.,6.],
  ];

  let x = array![1.,2.,3.];
  let y = array![4.,5.,6.];

  println!("The ndarray x: {}", x.view());
  println!("The ndarray y: {}", y.view());

  println!("X + Y: {}", &x+&y);
  println!("X * Y: {}", &x*&y);
  println!("X dot Y: {}", &x.dot(&y));
  
  // test square
  println!("square x: {}", square(&x));
  //println!("square x: {}", square(x));
  // test max
//  println!("maximum x y: {}", maximum(&x,&y));

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

  assert_eq!(a.ndim(), 2);
  assert_eq!(a.len(), 6);
  assert_eq!(a.shape(), [2, 3]);
  assert_eq!(a.is_empty(), false);

  println!("{:?}", a);


}
