#![allow(clippy::if_same_then_else)]
#![allow(clippy::never_loop)]
#![allow(clippy::almost_swapped)]
#![allow(non_camel_case_types)]

/**
 * Stemmer API derived from https://github.com/CurrySoftware/rust-stemmers :
 * ---
 * MIT License
 *
 * Copyright (c) 2017 Jakob Demler
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 * ---
 *
 * Snowball module generated from https://github.com/snowballstem/snowball :
 * ---
 * Copyright (c) 2001, Dr Martin Porter
 * Copyright (c) 2004,2005, Richard Boulton
 * Copyright (c) 2013, Yoshiki Shibukawa
 * Copyright (c) 2006,2007,2009,2010,2011,2014-2019, Olly Betts
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 *   1. Redistributions of source code must retain the above copyright notice,
 *      this list of conditions and the following disclaimer.
 *   2. Redistributions in binary form must reproduce the above copyright notice,
 *      this list of conditions and the following disclaimer in the documentation
 *      and/or other materials provided with the distribution.
 *   3. Neither the name of the Snowball project nor the names of its contributors
 *      may be used to endorse or promote products derived from this software
 *      without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
 * ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 * ---
 */
use std::borrow::Cow;

mod snowball;

use snowball::algorithms::get_algorithm;
pub use snowball::algorithms::Algorithm;
use snowball::SnowballEnv;

mod errors {
    use std::fmt;

    #[derive(Debug)]
    pub struct NoAlgorithm;

    impl fmt::Display for NoAlgorithm {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "pagefind_stem was compiled with no algorithm feature flags set. Nothing to do."
            )
        }
    }

    impl std::error::Error for NoAlgorithm {}
}

/// Wrapps a usable interface around the actual stemmer implementation
pub struct Stemmer {
    stemmer: fn(&mut SnowballEnv) -> bool,
}

impl Stemmer {
    /// Create a new stemmer from an algorithm
    pub fn create(lang: Algorithm) -> Self {
        Self {
            stemmer: lang.into(),
        }
    }

    /// Try to create a new stemmer from whatever algorithm this crate was compiled with.
    /// Returns an error if this crate was compiled with no feature flags.
    /// Returns an algorithm at random if this crate was compiled with multiple feature flags.
    pub fn try_create_default() -> Result<Self, errors::NoAlgorithm> {
        match get_algorithm() {
            Some(algo) => Ok(Self { stemmer: algo }),
            None => Err(errors::NoAlgorithm),
        }
    }

    /// Stem a single word
    /// Please note, that the input is expected to be all lowercase (if that is applicable).
    pub fn stem<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let mut env = SnowballEnv::create(input);
        (self.stemmer)(&mut env);
        env.get_current()
    }
}
