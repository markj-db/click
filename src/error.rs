// Copyright 2017 Databricks, Inc.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


use hyper;
use std::{error, fmt, io};
use std::convert::From;

use serde_json;

#[derive(Debug)]
pub enum KubeError {
    Io(io::Error),
    HyperParse(hyper::error::ParseError),
    HyperErr(hyper::error::Error),
    SerdeJson(serde_json::Error),
}


impl fmt::Display for KubeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KubeError::Io(ref err) => write!(f, "IO error: {}", err),
            KubeError::HyperParse(ref err) => write!(f, "Hyper parse error: {}", err),
            KubeError::HyperErr(ref err) => write!(f, "Hyper error: {}", err),
            KubeError::SerdeJson(ref err) => write!(f, "Serde json error: {}", err),
        }
    }
}

impl error::Error for KubeError {
    fn description(&self) -> &str {
        match *self {
            KubeError::Io(ref err) => err.description(),
            KubeError::HyperParse(ref err) => err.description(),
            KubeError::HyperErr(ref err) => err.description(),
            KubeError::SerdeJson(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            KubeError::Io(ref err) => Some(err),
            KubeError::HyperParse(ref err) => Some(err),
            KubeError::HyperErr(ref err) => Some(err),
            KubeError::SerdeJson(ref err) => Some(err),
        }
    }
}


impl From<io::Error> for KubeError {
    fn from(err: io::Error) -> KubeError {
        KubeError::Io(err)
    }
}

impl From<hyper::error::ParseError> for KubeError {
    fn from(err: hyper::error::ParseError) -> KubeError {
        KubeError::HyperParse(err)
    }
}


impl From<hyper::error::Error> for KubeError {
    fn from(err: hyper::error::Error) -> KubeError {
        KubeError::HyperErr(err)
    }
}

impl From<serde_json::Error> for KubeError {
    fn from(err: serde_json::Error) -> KubeError {
        KubeError::SerdeJson(err)
    }
}