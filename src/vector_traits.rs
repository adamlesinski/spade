// Copyright 2016 The Spade Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


use nalgebra as na;
use cgmath as cg;
use nalgebra::{Repeat};

use std::fmt::Debug;
use traits::SpadeNum;
use num::{zero};
use misc::{min_inline, max_inline};

/// Abstraction over vectors with a fixed number of dimensions.
/// Spade will work with any vector type implementing this trait, at the
/// moment vectors of the `cgmath` and `nalgebra` crates are supported.
/// Also, the trait is implemented for fixed arrays of length 2, 3 and 4, allowing
/// to use spade's datastructures with fixed size arrays as point coordinates.
/// That means that the trait's methods are also implemented for
/// these array types, thus be careful when importing `VectorN`.
///
/// Implement this if you want spade to support your own vector types.
/// Also consider adding an (empty) implementation of `TwoDimensional`
/// or `ThreeDimensional` if appropriate.
pub trait VectorN
    where Self: Clone,
          Self: Debug,
          Self: PartialEq {
    /// The vector's internal scalar type.
    type Scalar: SpadeNum;

    /// The (fixed) number of dimensions of this vector type.
    fn dimensions() -> usize;

    /// Creates a new vector with all compoenents set to a certain value.
    fn from_value(value: Self::Scalar) -> Self;

    /// Returns the nth element of this vector.
    fn nth(&self, index: usize) -> &Self::Scalar;
    /// Returns a mutable reference to the nth element of this vector.
    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar;
}

/// Adds some private methods to the ```VectorN``` trait.
pub trait VectorNExtensions : VectorN {
    /// Creates a new vector with all components initialized to zero.
    fn new() -> Self {
        Self::from_value(zero())
    }

    /// Adds two vectors.
    fn add(&self, rhs: &Self) -> Self {
        self.component_wise(rhs, |l, r| l + r)
    }

    /// Substracts two vectors.
    fn sub(&self, rhs: &Self) -> Self {
        self.component_wise(rhs, |l, r| l - r)
    }

    /// Divides this vector with a scalar value.
    fn div(&self, scalar: Self::Scalar) -> Self {
        self.map(|x| x / scalar.clone())
    }


    /// Multiplies this vector with a scalar value.
    fn mul(&self, scalar: Self::Scalar) -> Self {
        self.map(|x| x * scalar.clone())
    }

    /// Applies a binary operation component wise.
    fn component_wise<F: Fn(Self::Scalar, Self::Scalar) -> Self::Scalar>(&self, rhs: &Self, f: F) -> Self {
        let mut result = self.clone();
        for i in 0 .. Self::dimensions() {
            *result.nth_mut(i) = f(self.nth(i).clone(), rhs.nth(i).clone());
        }
        result
    }

    /// Maps an unary operation to all compoenents.
    fn map<F: Fn(Self::Scalar) -> O::Scalar, O: VectorN>(&self, f: F) -> O {
        let mut result = O::new();
        for i in 0 .. Self::dimensions() {
            *result.nth_mut(i)  = f(self.nth(i).clone());
        }
        result
    }

    /// Returns a new vector containing the minimum values of this and another vector (componentwise)
    fn min_vec(&self, rhs: &Self) -> Self {
        self.component_wise(rhs, |l, r| min_inline(l, r))
    }

    /// Returns a new vector containing the maximum values of this and another vector (componentwise)
    fn max_vec(&self, rhs: &Self) -> Self {
        self.component_wise(rhs, |l, r| max_inline(l, r))
    }

    /// Fold operation over all vector components.
    fn fold<T, F: Fn(T, Self::Scalar) -> T>(&self, mut acc: T, f: F) -> T {
        for i in 0 .. Self::dimensions() {
            acc = f(acc, self.nth(i).clone());
        }
        acc
    }

    /// Checks if a property holds for all components of this and another vector.
    fn all_comp_wise<F: Fn(Self::Scalar, Self::Scalar) -> bool>(&self, rhs: &Self, f: F) -> bool {
        for i in 0 .. Self::dimensions() {
            if !f(self.nth(i).clone(), rhs.nth(i).clone()) {
                return false;
            }
        }
        true
    }

    /// Returns the vector's dot product.
    fn dot(&self, rhs: &Self) -> Self::Scalar {
        self.component_wise(rhs, |l, r| l * r).fold(zero(), |acc, val| acc + val)
    }

    /// Returns the vector's squared length.
    fn length2(&self) -> Self::Scalar {
        self.dot(&self)
    }
}

impl <T> VectorNExtensions for T where T: VectorN { }

/// A two dimensional Vector.
/// Some datastructures will only work if two dimensional vectors are given,
/// this trait makes sure that only such vectors can be passed.
pub trait TwoDimensional : VectorN { }

impl <S: SpadeNum + cg::BaseNum> TwoDimensional for cg::Vector2<S> { }
impl <S: SpadeNum + na::BaseNum> TwoDimensional for na::Vector2<S> { }
impl <S: SpadeNum + Copy> TwoDimensional for [S; 2] { }

/// A three dimensional Vector.
/// Some algorithms will only work with three dimensional vectors, this trait makes
/// sure that only such vectors can be used.
pub trait ThreeDimensional : VectorN {
    /// The cross product of this vector and another.
    fn cross(&self, other: &Self) -> Self {
        let mut result = Self::new();
        *result.nth_mut(0) = self.nth(1).clone() * other.nth(2).clone() 
            - self.nth(2).clone() * other.nth(1).clone();
        *result.nth_mut(1) = self.nth(2).clone() * other.nth(0).clone()
            - self.nth(0).clone() * other.nth(2).clone();
        *result.nth_mut(2) = self.nth(0).clone() * other.nth(1).clone()
            - self.nth(1).clone() * other.nth(0).clone();
        result
    }
}

impl <S: SpadeNum + cg::BaseNum> ThreeDimensional for cg::Vector3<S> { }

impl <S: SpadeNum + na::BaseNum> ThreeDimensional for na::Vector3<S> { }

impl <S: SpadeNum + Copy> ThreeDimensional for [S; 3] { }

impl <S: SpadeNum + Copy> VectorN for [S; 2] {
    type Scalar = S;
    fn dimensions() -> usize { 2 }

    fn nth(&self, index: usize) -> &S { &self[index] }
    fn nth_mut(&mut self, index: usize) -> &mut S { &mut self[index] }
    
    fn from_value(value: Self::Scalar) -> Self {
        [value; 2]
    }
}

impl <S: SpadeNum + Copy> VectorN for [S; 3] {
    type Scalar = S;
    fn dimensions() -> usize { 3 }

    fn nth(&self, index: usize) -> &S { &self[index] }
    fn nth_mut(&mut self, index: usize) -> &mut S { &mut self[index] }
    
    fn from_value(value: Self::Scalar) -> Self {
        [value; 3]
    }
}

impl <S: SpadeNum + Copy> VectorN for [S; 4] {
    type Scalar = S;
    
    fn dimensions() -> usize { 4 }

    fn nth(&self, index: usize) -> &S { &self[index] }
    fn nth_mut(&mut self, index: usize) -> &mut S { &mut self[index] }
    
    fn from_value(value: Self::Scalar) -> Self {
        [value; 4]
    }
}

impl<S: SpadeNum + cg::BaseNum> VectorN for cg::Vector2<S> {
    type Scalar = S;
    
    fn dimensions() -> usize { 2 }

    fn nth(&self, index: usize) -> &S { &self[index] }
    fn nth_mut(&mut self, index: usize) -> &mut S { &mut self[index] }

    fn from_value(value: Self::Scalar) -> Self {
        cg::Array::from_value(value)
    }
}

impl<S: SpadeNum + cg::BaseNum> VectorN for cg::Vector3<S> {
    type Scalar = S;
    
    fn dimensions() -> usize { 3 }

    fn nth(&self, index: usize) -> &S { &self[index] }
    fn nth_mut(&mut self, index: usize) -> &mut S { &mut self[index] }

    fn from_value(value: Self::Scalar) -> Self {
        cg::Array::from_value(value)
    }
}

impl<S: SpadeNum + cg::BaseNum> VectorN for cg::Vector4<S> {
    type Scalar = S;
    
    fn dimensions() -> usize { 4 }

    fn nth(&self, index: usize) -> &S { &self[index] }
    fn nth_mut(&mut self, index: usize) -> &mut S { &mut self[index] }

    fn from_value(value: Self::Scalar) -> Self {
        cg::Array::from_value(value)
    }
}

impl<S: SpadeNum + na::BaseNum> VectorN for na::Vector2<S> {
    type Scalar = S;
    
    fn dimensions() -> usize { 2 }

    fn nth(&self, index: usize) -> &S { &self[index] }
    fn nth_mut(&mut self, index: usize) -> &mut S { &mut self[index] }

    fn from_value(value: Self::Scalar) -> Self {
        na::Vector2::repeat(value)
    }
}

impl<S: SpadeNum + na::BaseNum> VectorN for na::Vector3<S> {
    type Scalar = S;
    
    fn dimensions() -> usize { 3 }

    fn nth(&self, index: usize) -> &S { &self[index] }
    fn nth_mut(&mut self, index: usize) -> &mut S { &mut self[index] }

    fn from_value(value: Self::Scalar) -> Self {
        na::Vector3::repeat(value)
    }
}

impl<S: SpadeNum + na::BaseNum> VectorN for na::Vector4<S> {
    type Scalar = S;
    
    fn dimensions() -> usize { 4 }

    fn nth(&self, index: usize) -> &S { &self[index] }
    fn nth_mut(&mut self, index: usize) -> &mut S { &mut self[index] }

    fn from_value(value: Self::Scalar) -> Self {
        na::Vector4::repeat(value)
    }
}
