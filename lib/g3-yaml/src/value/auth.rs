/*
 * Copyright 2023 ByteDance and/or its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use anyhow::anyhow;
use yaml_rust::Yaml;

use g3_types::auth::{Password, Username};

pub fn as_username(value: &Yaml) -> anyhow::Result<Username> {
    if let Yaml::String(s) = value {
        Ok(Username::from_original(s)?)
    } else {
        Err(anyhow!("yaml value type for username should be string"))
    }
}

pub fn as_password(value: &Yaml) -> anyhow::Result<Password> {
    if let Yaml::String(s) = value {
        Ok(Password::from_original(s)?)
    } else {
        Err(anyhow!("yaml value type for password should be string"))
    }
}
