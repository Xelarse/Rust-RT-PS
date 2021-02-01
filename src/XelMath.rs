pub mod vector
{
    use std::ops::{Add, Sub, Mul, Div};

    #[derive(Copy, Clone, Debug)]
    pub struct Vec3
    {
        _e: [f64; 3]
    }

    #[allow(dead_code)]
    impl Vec3
    {
        pub fn new(x: f64, y: f64, z: f64) -> Vec3
        {
            return Vec3 {_e: [x,y,z]};
        }

        pub fn x(self) -> f64
        {
            return self._e[0];
        }

        pub fn y(self) -> f64
        {
            return self._e[1];
        }

        pub fn z(self) -> f64
        {
            return self._e[2];
        }

        pub fn set_x(&mut self, rhs: f64)
        {
            self._e[0] = rhs;
        }

        pub fn set_y(&mut self, rhs: f64)
        {
            self._e[1] = rhs;
        }

        pub fn set_z(&mut self, rhs: f64)
        {
            self._e[2] = rhs;
        }

        pub fn len_sq(self) -> f64
        {
            return (self._e[0] * self._e[0]) + (self._e[1] * self._e[1]) + (self._e[2] * self._e[2]);
        }

        pub fn len(self) -> f64
        {
            return self.len_sq().sqrt();
        }

        pub fn dot(self, rhs: Vec3) -> f64
        {
            return (self._e[0] * rhs._e[0]) + (self._e[1] * rhs._e[1]) + (self._e[2] * rhs._e[2]);
        }

        pub fn cross(self, rhs: Vec3) -> Vec3
        {
            return Vec3{
                _e: [
                    (self._e[1] * rhs._e[2]) - (self._e[2] * rhs._e[1]),
                    (self._e[2] * rhs._e[0]) - (self._e[0] * rhs._e[2]),
                    (self._e[0] * rhs._e[1]) - (self._e[1] * rhs._e[0])
                ]
            };
        }

        pub fn unit_vec(self) -> Vec3
        {
            return self / self.len();
        }

        //Static Funcs
        pub fn hdr_to_rgb(lhs: Vec3) -> Vec3
        {
            return Vec3{
                _e: [
                    255.999 * lhs._e[0],
                    255.999 * lhs._e[1],
                    255.999 * lhs._e[2]
                ]
            };
        }

        pub fn rgb_to_hdr(lhs: Vec3) -> Vec3
        {
            return Vec3{
                _e: [
                    255.999 * lhs._e[0],
                    255.999 * lhs._e[1],
                    255.999 * lhs._e[2]
                ]
            };
        }
    }

    //Vec3 operators
    impl Add for Vec3
    {
        type Output = Self;
        
        fn add(self, rhs: Self) -> Self
        {
            Self {_e: [self._e[0] + rhs._e[0], self._e[1] + rhs._e[1], self._e[2] + rhs._e[2]]}
        }
    }

    impl Sub for Vec3
    {
        type Output = Self;
        
        fn sub(self, rhs: Self) -> Self
        {
            Self {_e: [self._e[0] - rhs._e[0], self._e[1] - rhs._e[1], self._e[2] - rhs._e[2]]}
        }
    }

    //Used for Multiplying Vec3 by another Vec3 aka (Vec3 * Vec3)
    impl Mul for Vec3
    {
        type Output = Self;
        
        fn mul(self, rhs: Self) -> Self
        {
            Self {_e: [self._e[0] * rhs._e[0], self._e[1] * rhs._e[1], self._e[2] * rhs._e[2]]}
        }
    }

    //Used for Multiplying Vec3 by a f64 SPECIFICALLY (Vec3 * f64) and not (f64 * Vec3)
    //Therefore whatever after the for is lhs and the type in the angled brackets is rhs. 
    impl Mul<f64> for Vec3
    {
        type Output = Self;
        
        fn mul(self, rhs: f64) -> Self
        {
            Self {_e: [self._e[0] * rhs, self._e[1] * rhs, self._e[2] * rhs]}
        }
    }

    impl Div for Vec3
    {
        type Output = Self;
        
        fn div(self, rhs: Self) -> Self
        {
            Self {_e: [self._e[0] / rhs._e[0], self._e[1] / rhs._e[1], self._e[2] / rhs._e[2]]}
        }
    }

    impl Div<f64> for Vec3
    {
        type Output = Self;
        
        fn div(self, rhs: f64) -> Self
        {
            let ratio = 1.0 / rhs;
            Self {_e: [self._e[0] * ratio, self._e[1] * ratio, self._e[2] * ratio]}
        }
    }
}

pub mod ray
{
    use super::vector::Vec3;

    #[derive(Copy, Clone, Debug)]
    struct Ray
    {
        ori: Vec3,
        dir: Vec3
    }

    #[allow(dead_code)]
    impl Ray
    {
        pub fn new(origin: Vec3, direction: Vec3) -> Ray
        {
            return Ray{ori: origin, dir: direction};
        }

        pub fn at(self, t: f64) -> Vec3
        {
            return self.ori + (self.dir * t);
        }
    }
}


// pub mod vector
// {
//     use std::ops::{Add, Sub, Mul, Div};

//     #[derive(Copy, Clone, Debug)]
//     pub struct Vec3<T> 
//     {
//         _x: T,
//         _y: T,
//         _z: T
//     }

//     #[allow(dead_code)]
//     impl<T> Vec3<T>
//     {
//         pub fn new(x: T, y: T, z: T) -> Vec3<T>
//         {
//             return Vec3 {_x: x, _y: y, _z: z};
//         }

//         pub fn get_x(self) -> T
//         {
//             return self._x;
//         }

//         pub fn get_y(self) -> T
//         {
//             return self._y;
//         }

//         pub fn get_z(self) -> T
//         {
//             return self._z;
//         }

//         pub fn set_x(&mut self, rhs: T)
//         {
//             self._x = rhs;
//         }

//         pub fn set_y(&mut self, rhs: T)
//         {
//             self._y = rhs;
//         }

//         pub fn set_z(&mut self, rhs: T)
//         {
//             self._z = rhs;
//         }
//     }

//     //Vec3 operators
//     impl<T: Add<Output = T>> Add for Vec3<T>
//     {
//         type Output = Self;
        
//         fn add(self, rhs: Self) -> Self::Output
//         {
//             Self {_x: self._x + rhs._x, _y: self._y + rhs._y, _z: self._z + rhs._z}
//         }
//     }

//     impl<T: Sub<Output = T>> Sub for Vec3<T>
//     {
//         type Output = Self;
        
//         fn sub(self, rhs: Self) -> Self::Output
//         {
//             Self {_x: self._x - rhs._x, _y: self._y - rhs._y, _z: self._z - rhs._z}
//         }
//     }

//     impl<T: Mul<Output = T>> Mul for Vec3<T>
//     {
//         type Output = Self;
        
//         fn mul(self, rhs: Self) -> Self::Output
//         {
//             Self {_x: self._x * rhs._x, _y: self._y * rhs._y, _z: self._z * rhs._z}
//         }
//     }

//     impl<T: Div<Output = T>> Div for Vec3<T>
//     {
//         type Output = Self;
        
//         fn div(self, rhs: Self) -> Self::Output
//         {
//             Self {_x: self._x / rhs._x, _y: self._y / rhs._y, _z: self._z / rhs._z}
//         }
//     }

//     //// Tried to use AddAssign but apparently cant be used with generics
//     // impl<T: std::ops::AddAssign<T>> std::ops::AddAssign for Vec3<T>
//     // {
//     //     fn add_assign(&mut self, rhs: Self)
//     //     {
//     //         *self = Self {
//     //             _x: self._x + rhs._x,
//     //             _y: self._y + rhs._y,
//     //             _z: self._z + rhs._z
//     //         }
//     //     }
//     // }
// }

// pub mod colour
// {
//     use crate::XelMath::vector;
//     use std::ops::{Add, Sub, Mul, Div};
//     use std::num;

//     struct xel_rgb
//     {
//         pub col: vector::Vec3<f64>
//     }

//     //operators
//     impl Add for xel_rgb
//     {
//         type Output = Self;
        
//         fn add(self, rhs: Self) -> Self
//         {
//             Self
//             {
//                 col: self.col + rhs.col
//             }
//         }
//     }

//     impl Sub for xel_rgb
//     {
//         type Output = Self;
        
//         fn sub(self, rhs: Self) -> Self
//         {
//             Self
//             {
//                 col: self.col - rhs.col
//             }
//         }
//     }

//     impl Mul for xel_rgb
//     {
//         type Output = Self;
        
//         fn mul(self, rhs: Self) -> Self
//         {
//             Self
//             {
//                 col: self.col * rhs.col
//             }
//         }
//     }

//     impl Div for xel_rgb
//     {
//         type Output = Self;
        
//         fn div(self, rhs: Self) -> Self
//         {
//             Self
//             {
//                 col: self.col / rhs.col
//             }
//         }
//     }
// }