// Copyright 2023 Google LLC
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

//! New rust RemoteAuth JNI library.
//!
//! This library takes the JNI calls from RemoteAuthService to the remoteauth protocol library
//! and from protocol library to platform (Java interface)

mod jnames;
mod unique_jvm;
mod utils;

pub mod remoteauth_jni_android_platform;
pub mod remoteauth_jni_android_protocol;
