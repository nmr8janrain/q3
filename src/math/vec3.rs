/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: math/vec3.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A 3D vector with X, Y, and Z components.
*/

pub use self::macro::Vec3f;

macro_rules! declare
(
  ($Type:ident, $Component:ty) =>
  (
    mod macro
    {
      pub struct $Type
      {
        x: $Component,
        y: $Component,
        z: $Component
      }

      impl $Type
      {
        pub fn new(nx: $Component, ny: $Component, nz: $Component) -> $Type
        { $Type{ x: nx, y: ny, z: nz } }

        pub fn zero() -> $Type
        { $Type{ x: 0 as $Component, y: 0 as $Component, z: 0 as $Component } }

        pub fn normalize(&mut self)
        {
          let mut len = self.length();

          if len < 0.0001 as $Component && len > -0.0001 as $Component /* TODO: Egh, hack. */
          { len = 1.0; } /* TODO: Return? */

          self.x /= len;
          self.y /= len;
          self.z /= len;
        }

        pub fn length(&self) -> $Component
        { float::sqrt(( (self.x * self.x) + 
                        (self.y * self.y) + 
                        (self.z * self.z)) as float) as $Component }

        pub fn cross(&self, rhs: &$Type) -> $Type
        {
          $Type { x: (self.y * rhs.z) - (self.z * rhs.y),
                  y: (self.z * rhs.x) - (self.x * rhs.z),
                  z: (self.x * rhs.y) - (self.y * rhs.x) }
        }

        pub fn dot(&self, rhs: &$Type) -> $Component
        { (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) }

        pub unsafe fn to_ptr(&self) -> *$Type
        { ptr::addr_of(self) }
      }

      /***** Operator Overloads *****/
      impl Add<$Type, $Type> for $Type
      {
        fn add(&self, rhs: &$Type) -> $Type
        {
          $Type{x: ( self.x + rhs.x ),
                y: ( self.y + rhs.y ),
                z: ( self.z + rhs.z ) }
        }
      }

      impl Sub<$Type, $Type> for $Type
      {
        fn sub(&self, rhs: &$Type) -> $Type
        {
          $Type{x: ( self.x - rhs.x ),
                y: ( self.y - rhs.y ),
                z: ( self.z - rhs.z ) }
        }
      }

      impl Mul<$Component, $Type> for $Type
      {
        fn mul(&self, rhs: &$Component) -> $Type
        {
          $Type{x: ( self.x * *rhs ),
                y: ( self.y * *rhs ),
                z: ( self.z * *rhs ) }
        }
      }

      impl Neg<$Type> for $Type
      {
        fn neg(&self) -> $Type
        {
          $Type{x: ( -self.x ),
                y: ( -self.y ),
                z: ( -self.z ) }
        }
      }
    }
  );
)

declare!(Vec3f, f32)

