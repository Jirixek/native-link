// Copyright 2022 The Native Link Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// The full version of a given tool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SemVer {
    /// The major version, e.g 10 for 10.2.3.
    #[prost(int32, tag = "1")]
    pub major: i32,
    /// The minor version, e.g. 2 for 10.2.3.
    #[prost(int32, tag = "2")]
    pub minor: i32,
    /// The patch version, e.g 3 for 10.2.3.
    #[prost(int32, tag = "3")]
    pub patch: i32,
    /// The pre-release version. Either this field or major/minor/patch fields
    /// must be filled. They are mutually exclusive. Pre-release versions are
    /// assumed to be earlier than any released versions.
    #[prost(string, tag = "4")]
    pub prerelease: ::prost::alloc::string::String,
}
