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

use actor::prelude::*;
use messaging::DeliverMessage;

actor_handlers!{ messaging::OP_DELIVER_MESSAGE => handle_message, 
                 core::OP_HEALTH_REQUEST => health }

fn handle_message(ctx: &CapabilitiesContext, msg: DeliverMessage) -> CallResult {
    ctx.println(&format!("Received message broker message: {:?}", msg));
    Ok(vec![])
}

fn health(_ctx: &CapabilitiesContext, _req: core::HealthRequest) -> ReceiveResult {
    Ok(vec![])
}
