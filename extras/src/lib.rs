// Copyright 2015-2019 Capital One Services, LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate wascc_actor as actor;

#[macro_use]
extern crate serde_json;

use actor::prelude::*;

actor_handlers! { http::OP_HANDLE_REQUEST => display_extras, core::OP_HEALTH_REQUEST => health }

fn display_extras(ctx: &CapabilitiesContext, _payload: http::Request) -> ReceiveResult {
    let result = json!(
    { "random": ctx.extras().get_random(0, 100)?,
      "guid" : ctx.extras().get_guid()?,
      "sequence": ctx.extras().get_sequence_number()?,
    });
    Ok(serialize(http::Response::json(result, 200, "OK"))?)
}

fn health(_ctx: &CapabilitiesContext, _payload: core::HealthRequest) -> ReceiveResult {
    Ok(vec![])
}
