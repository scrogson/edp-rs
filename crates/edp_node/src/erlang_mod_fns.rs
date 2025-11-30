// Copyright (C) 2025-2026 Michael S. Klishin and Contributors
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

use crate::errors::Result;
use crate::node::Node;
use erltf::OwnedTerm;
use erltf::types::Atom;

impl Node {
    pub async fn erlang_system_info(&self, remote_node: &str, item: &str) -> Result<OwnedTerm> {
        self.rpc_call(
            remote_node,
            "erlang",
            "system_info",
            vec![OwnedTerm::Atom(Atom::new(item))],
        )
        .await
    }

    pub async fn erlang_statistics(&self, remote_node: &str, item: &str) -> Result<OwnedTerm> {
        self.rpc_call(
            remote_node,
            "erlang",
            "statistics",
            vec![OwnedTerm::Atom(Atom::new(item))],
        )
        .await
    }

    pub async fn erlang_memory(&self, remote_node: &str) -> Result<OwnedTerm> {
        self.rpc_call(remote_node, "erlang", "memory", vec![]).await
    }

    pub async fn erlang_processes(&self, remote_node: &str) -> Result<OwnedTerm> {
        self.rpc_call(remote_node, "erlang", "processes", vec![])
            .await
    }

    pub async fn erlang_process_info(
        &self,
        remote_node: &str,
        pid: OwnedTerm,
        items: Vec<Atom>,
    ) -> Result<OwnedTerm> {
        let items_list = OwnedTerm::List(items.into_iter().map(OwnedTerm::Atom).collect());
        self.rpc_call(remote_node, "erlang", "process_info", vec![pid, items_list])
            .await
    }

    pub async fn erlang_list_to_pid(&self, remote_node: &str, pid_str: &str) -> Result<OwnedTerm> {
        self.rpc_call(
            remote_node,
            "erlang",
            "list_to_pid",
            vec![OwnedTerm::charlist(pid_str)],
        )
        .await
    }
}
