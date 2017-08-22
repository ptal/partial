// Copyright 2017 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! `Environment` is constituted of a state named `env` that is never lost (even if the partial data is equal to `Nothing`).
//! It behaves similarly to `Partial` but the binders (e.g. `and_then`) take two parameters: the environment and the unwrapped partial data.
//! This structure is useful to implement compiler where `env` contains the configuration and `data` the compilation context.

use Partial;
use Partial::*;
use std::ops::{Deref, DerefMut};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Environment<T, U>
{
  env: T,
  data: Partial<U>
}

impl<T, U> Environment<T, U>
{
  pub fn new(env: T, data: Partial<U>) -> Self {
    Environment {
      env: env,
      data: data
    }
  }

  pub fn value(env: T, data: U) -> Self {
    Self::new(env, Partial::Value(data))
  }

  pub fn fake(env: T, data: U) -> Self {
    Self::new(env, Partial::Fake(data))
  }

  pub fn nothing(env: T) -> Self {
    Self::new(env, Partial::Nothing)
  }

  pub fn unwrap(self) -> U {
    self.data.unwrap()
  }

  pub fn expect(self, msg: &str) -> U {
    self.data.expect(msg)
  }

  pub fn unwrap_or_else<F>(self, f: F) -> U where
   F: FnOnce() -> U
  {
    self.data.unwrap_or_else(f)
  }

  pub fn and_then<R, F: FnOnce(T, U) -> Environment<T, R>>(self, f: F) -> Environment<T, R> {
    match self.data {
      Value(x) => f(self.env, x),
      Fake(x) => {
        let r = f(self.env, x);
        match r.data {
          Value(x) => Environment::fake(r.env, x),
          x => Environment::new(r.env, x)
        }
      }
      Nothing => Environment::nothing(self.env)
    }
  }

  pub fn and_next<R, F: FnOnce(T, U) -> Environment<T, R>>(self, f: F) -> Environment<T, R> {
    match self.data {
      Value(x) => f(self.env, x),
      _ => Environment::nothing(self.env)
    }
  }
}

impl<T, U> Deref for Environment<T, U> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.env
  }
}

impl<T, U> DerefMut for Environment<T, U> {
  fn deref_mut(&mut self) -> &mut T {
    &mut self.env
  }
}
