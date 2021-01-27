pub mod vector
{
    #[derive(Copy, Clone)]
    pub struct Vec3<T> 
    {
        x: T,
        y: T,
        z: T
    }

    impl<T> Vec3<T>
    {
        pub fn new(x: T, y: T, z: T) -> Vec3<T>
        {
            return Vec3 {x: x, y: y, z: z};
        }

        pub fn x(self) -> T
        {
            return self.x;
        }

        pub fn y(self) -> T
        {
            return self.y;
        }

        pub fn z(self) -> T
        {
            return self.z;
        }
    }
}