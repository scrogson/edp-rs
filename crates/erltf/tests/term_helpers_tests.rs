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

use erltf::OwnedTerm;
use erltf::types::{Atom, ExternalPid, Mfa};
use erltf::{erl_atom, erl_atoms, erl_int, erl_list, erl_map, erl_tuple};
use std::collections::BTreeMap;

#[test]
fn test_proplist_get_finds_value() {
    let proplist = OwnedTerm::List(vec![
        OwnedTerm::Tuple(vec![
            OwnedTerm::Atom(Atom::new("name")),
            OwnedTerm::String("Alice".to_string()),
        ]),
        OwnedTerm::Tuple(vec![
            OwnedTerm::Atom(Atom::new("age")),
            OwnedTerm::Integer(30),
        ]),
        OwnedTerm::Tuple(vec![
            OwnedTerm::Atom(Atom::new("city")),
            OwnedTerm::String("Paris".to_string()),
        ]),
    ]);

    assert_eq!(
        proplist.proplist_get_atom_key("name"),
        Some(&OwnedTerm::String("Alice".to_string()))
    );
    assert_eq!(
        proplist.proplist_get_atom_key("age"),
        Some(&OwnedTerm::Integer(30))
    );
    assert_eq!(
        proplist.proplist_get_atom_key("city"),
        Some(&OwnedTerm::String("Paris".to_string()))
    );
}

#[test]
fn test_proplist_get_not_found() {
    let proplist = OwnedTerm::List(vec![OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("name")),
        OwnedTerm::String("Bob".to_string()),
    ])]);

    assert_eq!(proplist.proplist_get_atom_key("nonexistent"), None);
}

#[test]
fn test_proplist_get_empty_list() {
    let proplist = OwnedTerm::List(vec![]);
    assert_eq!(proplist.proplist_get_atom_key("anything"), None);
}

#[test]
fn test_proplist_get_on_non_list() {
    let not_a_list = OwnedTerm::Integer(42);
    assert_eq!(not_a_list.proplist_get_atom_key("key"), None);
}

#[test]
fn test_proplist_get_malformed_tuples() {
    let proplist = OwnedTerm::List(vec![
        OwnedTerm::Tuple(vec![
            OwnedTerm::Atom(Atom::new("valid")),
            OwnedTerm::Integer(1),
        ]),
        OwnedTerm::Tuple(vec![
            OwnedTerm::Atom(Atom::new("too_many")),
            OwnedTerm::Integer(2),
            OwnedTerm::Integer(3),
        ]),
        OwnedTerm::Tuple(vec![OwnedTerm::Atom(Atom::new("lonely"))]),
        OwnedTerm::Integer(42),
    ]);

    assert_eq!(
        proplist.proplist_get_atom_key("valid"),
        Some(&OwnedTerm::Integer(1))
    );
    assert_eq!(proplist.proplist_get_atom_key("too_many"), None);
    assert_eq!(proplist.proplist_get_atom_key("lonely"), None);
}

#[test]
fn test_map_get_atom_finds_value() {
    let mut map = BTreeMap::new();
    map.insert(
        OwnedTerm::Atom(Atom::new("name")),
        OwnedTerm::String("Charlie".to_string()),
    );
    map.insert(OwnedTerm::Atom(Atom::new("age")), OwnedTerm::Integer(25));
    map.insert(
        OwnedTerm::Atom(Atom::new("city")),
        OwnedTerm::String("London".to_string()),
    );
    let map_term = OwnedTerm::Map(map);

    assert_eq!(
        map_term.map_get_atom_key("name"),
        Some(&OwnedTerm::String("Charlie".to_string()))
    );
    assert_eq!(
        map_term.map_get_atom_key("age"),
        Some(&OwnedTerm::Integer(25))
    );
    assert_eq!(
        map_term.map_get_atom_key("city"),
        Some(&OwnedTerm::String("London".to_string()))
    );
}

#[test]
fn test_map_get_atom_not_found() {
    let mut map = BTreeMap::new();
    map.insert(OwnedTerm::Atom(Atom::new("key")), OwnedTerm::Integer(42));
    let map_term = OwnedTerm::Map(map);

    assert_eq!(map_term.map_get_atom_key("nonexistent"), None);
}

#[test]
fn test_map_get_atom_empty_map() {
    let map_term = OwnedTerm::Map(BTreeMap::new());
    assert_eq!(map_term.map_get_atom_key("anything"), None);
}

#[test]
fn test_map_get_atom_on_non_map() {
    let not_a_map = OwnedTerm::Integer(42);
    assert_eq!(not_a_map.map_get_atom_key("key"), None);
}

#[test]
fn test_as_erlang_string_from_integer_list() {
    let term = OwnedTerm::List(vec![
        OwnedTerm::Integer(72),
        OwnedTerm::Integer(101),
        OwnedTerm::Integer(108),
        OwnedTerm::Integer(108),
        OwnedTerm::Integer(111),
    ]);

    assert_eq!(term.as_erlang_string(), Some("Hello".to_string()));
}

#[test]
fn test_as_erlang_string_from_string() {
    let term = OwnedTerm::String("World".to_string());
    assert_eq!(term.as_erlang_string(), Some("World".to_string()));
}

#[test]
fn test_as_erlang_string_from_binary() {
    let term = OwnedTerm::Binary(vec![82, 117, 115, 116]);
    assert_eq!(term.as_erlang_string(), Some("Rust".to_string()));
}

#[test]
fn test_as_erlang_string_invalid_integer_list() {
    let term = OwnedTerm::List(vec![
        OwnedTerm::Integer(72),
        OwnedTerm::Integer(256),
        OwnedTerm::Integer(108),
    ]);

    assert_eq!(term.as_erlang_string(), None);
}

#[test]
fn test_as_erlang_string_mixed_list() {
    let term = OwnedTerm::List(vec![
        OwnedTerm::Integer(72),
        OwnedTerm::Atom(Atom::new("not_an_int")),
        OwnedTerm::Integer(108),
    ]);

    assert_eq!(term.as_erlang_string(), None);
}

#[test]
fn test_as_erlang_string_on_non_string_types() {
    assert_eq!(OwnedTerm::Integer(42).as_erlang_string(), None);
    assert_eq!(OwnedTerm::Atom(Atom::new("atom")).as_erlang_string(), None);
    assert_eq!(OwnedTerm::Float(2.5).as_erlang_string(), None);
}

#[test]
fn test_charlist_from_ascii() {
    let term = OwnedTerm::charlist("Hello");
    assert_eq!(
        term,
        OwnedTerm::List(vec![
            OwnedTerm::Integer(72),
            OwnedTerm::Integer(101),
            OwnedTerm::Integer(108),
            OwnedTerm::Integer(108),
            OwnedTerm::Integer(111),
        ])
    );
}

#[test]
fn test_charlist_from_unicode() {
    let term = OwnedTerm::charlist("日本");
    assert_eq!(
        term,
        OwnedTerm::List(vec![OwnedTerm::Integer(0x65E5), OwnedTerm::Integer(0x672C),])
    );
}

#[test]
fn test_charlist_empty() {
    let term = OwnedTerm::charlist("");
    assert_eq!(term, OwnedTerm::List(vec![]));
}

#[test]
fn test_is_charlist_valid() {
    let term = OwnedTerm::List(vec![
        OwnedTerm::Integer(72),
        OwnedTerm::Integer(101),
        OwnedTerm::Integer(108),
    ]);
    assert!(term.is_charlist());
}

#[test]
fn test_is_charlist_empty() {
    assert!(OwnedTerm::List(vec![]).is_charlist());
    assert!(OwnedTerm::Nil.is_charlist());
}

#[test]
fn test_is_charlist_negative_integer() {
    let term = OwnedTerm::List(vec![OwnedTerm::Integer(-1)]);
    assert!(!term.is_charlist());
}

#[test]
fn test_is_charlist_non_integer() {
    let term = OwnedTerm::List(vec![OwnedTerm::Atom(Atom::new("a"))]);
    assert!(!term.is_charlist());
}

#[test]
fn test_is_charlist_on_non_list() {
    assert!(!OwnedTerm::Integer(42).is_charlist());
    assert!(!OwnedTerm::Atom(Atom::new("atom")).is_charlist());
}

#[test]
fn test_as_pid() {
    let pid = ExternalPid::new(Atom::new("node@host"), 1, 2, 3);
    let term = OwnedTerm::Pid(pid.clone());
    assert_eq!(term.as_pid(), Some(&pid));
}

#[test]
fn test_as_pid_on_non_pid() {
    assert_eq!(OwnedTerm::Integer(42).as_pid(), None);
}

#[test]
fn test_try_as_pid() {
    let pid = ExternalPid::new(Atom::new("node@host"), 1, 2, 3);
    let term = OwnedTerm::Pid(pid.clone());
    assert_eq!(term.try_as_pid().unwrap(), &pid);
}

#[test]
fn test_try_as_pid_error() {
    let term = OwnedTerm::Integer(42);
    assert!(term.try_as_pid().is_err());
}

#[test]
fn test_is_pid() {
    let pid = ExternalPid::new(Atom::new("node@host"), 1, 2, 3);
    assert!(OwnedTerm::Pid(pid).is_pid());
    assert!(!OwnedTerm::Integer(42).is_pid());
}

#[test]
fn test_proplist_get_i64() {
    let proplist = OwnedTerm::List(vec![OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("count")),
        OwnedTerm::Integer(42),
    ])]);
    assert_eq!(proplist.proplist_get_i64("count"), Some(42));
    assert_eq!(proplist.proplist_get_i64("missing"), None);
}

#[test]
fn test_proplist_get_bool() {
    let proplist = OwnedTerm::List(vec![
        OwnedTerm::Tuple(vec![
            OwnedTerm::Atom(Atom::new("enabled")),
            OwnedTerm::Atom(Atom::new("true")),
        ]),
        OwnedTerm::Tuple(vec![
            OwnedTerm::Atom(Atom::new("disabled")),
            OwnedTerm::Atom(Atom::new("false")),
        ]),
    ]);
    assert_eq!(proplist.proplist_get_bool("enabled"), Some(true));
    assert_eq!(proplist.proplist_get_bool("disabled"), Some(false));
    assert_eq!(proplist.proplist_get_bool("missing"), None);
}

#[test]
fn test_proplist_get_atom() {
    let proplist = OwnedTerm::List(vec![OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("status")),
        OwnedTerm::Atom(Atom::new("running")),
    ])]);
    assert_eq!(
        proplist.proplist_get_atom("status"),
        Some(&Atom::new("running"))
    );
    assert_eq!(proplist.proplist_get_atom("missing"), None);
}

#[test]
fn test_proplist_get_string() {
    let proplist = OwnedTerm::List(vec![OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("name")),
        OwnedTerm::List(vec![
            OwnedTerm::Integer(66),
            OwnedTerm::Integer(111),
            OwnedTerm::Integer(98),
        ]),
    ])]);
    assert_eq!(
        proplist.proplist_get_string("name"),
        Some("Bob".to_string())
    );
    assert_eq!(proplist.proplist_get_string("missing"), None);
}

#[test]
fn test_proplist_get_pid() {
    let pid = ExternalPid::new(Atom::new("node@host"), 1, 2, 3);
    let proplist = OwnedTerm::List(vec![OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("process")),
        OwnedTerm::Pid(pid.clone()),
    ])]);
    assert_eq!(proplist.proplist_get_pid("process"), Some(&pid));
    assert_eq!(proplist.proplist_get_pid("missing"), None);
}

#[test]
fn test_pid_to_charlist_term() {
    let pid = ExternalPid::new(Atom::new("node@host"), 123, 456, 7);
    let charlist = pid.to_charlist_term();
    assert!(charlist.is_charlist());
    assert_eq!(charlist.as_erlang_string(), Some("<0.123.456>".to_string()));
}

#[test]
fn test_mfa_new() {
    let mfa = Mfa::new("erlang", "self", 0);
    assert_eq!(mfa.module, Atom::new("erlang"));
    assert_eq!(mfa.function, Atom::new("self"));
    assert_eq!(mfa.arity, 0);
}

#[test]
fn test_mfa_display() {
    let mfa = Mfa::new("lists", "map", 2);
    assert_eq!(format!("{}", mfa), "lists:map/2");
}

#[test]
fn test_mfa_to_term() {
    let mfa = Mfa::new("erlang", "node", 0);
    let term = mfa.to_term();
    assert_eq!(
        term,
        OwnedTerm::Tuple(vec![
            OwnedTerm::Atom(Atom::new("erlang")),
            OwnedTerm::Atom(Atom::new("node")),
            OwnedTerm::Integer(0),
        ])
    );
}

#[test]
fn test_mfa_try_from_term() {
    let term = OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("lists")),
        OwnedTerm::Atom(Atom::new("sort")),
        OwnedTerm::Integer(1),
    ]);
    let mfa = Mfa::try_from_term(&term).unwrap();
    assert_eq!(mfa.module, Atom::new("lists"));
    assert_eq!(mfa.function, Atom::new("sort"));
    assert_eq!(mfa.arity, 1);
}

#[test]
fn test_mfa_try_from_term_rejects_list_arity() {
    let term = OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("io")),
        OwnedTerm::Atom(Atom::new("format")),
        OwnedTerm::List(vec![
            OwnedTerm::String("~p~n".to_string()),
            OwnedTerm::List(vec![]),
        ]),
    ]);
    assert!(Mfa::try_from_term(&term).is_none());
}

#[test]
fn test_mfa_try_from_term_invalid() {
    assert!(Mfa::try_from_term(&OwnedTerm::Integer(42)).is_none());
    assert!(Mfa::try_from_term(&OwnedTerm::Tuple(vec![OwnedTerm::Integer(1)])).is_none());
}

#[test]
fn test_as_charlist_string_ascii() {
    let term = OwnedTerm::charlist("Hello");
    assert_eq!(term.as_charlist_string(), Some("Hello".to_string()));
}

#[test]
fn test_as_charlist_string_unicode() {
    let term = OwnedTerm::charlist("日本語");
    assert_eq!(term.as_charlist_string(), Some("日本語".to_string()));
}

#[test]
fn test_as_charlist_string_emoji() {
    let term = OwnedTerm::charlist("🦀");
    assert_eq!(term.as_charlist_string(), Some("🦀".to_string()));
}

#[test]
fn test_as_charlist_string_empty() {
    assert_eq!(
        OwnedTerm::List(vec![]).as_charlist_string(),
        Some(String::new())
    );
    assert_eq!(OwnedTerm::Nil.as_charlist_string(), Some(String::new()));
}

#[test]
fn test_as_charlist_string_from_string_term() {
    let term = OwnedTerm::String("test".to_string());
    assert_eq!(term.as_charlist_string(), Some("test".to_string()));
}

#[test]
fn test_as_charlist_string_from_binary() {
    let term = OwnedTerm::Binary(b"binary".to_vec());
    assert_eq!(term.as_charlist_string(), Some("binary".to_string()));
}

#[test]
fn test_as_charlist_string_invalid_codepoint() {
    let term = OwnedTerm::List(vec![OwnedTerm::Integer(0x110000)]);
    assert_eq!(term.as_charlist_string(), None);
}

#[test]
fn test_as_charlist_string_negative() {
    let term = OwnedTerm::List(vec![OwnedTerm::Integer(-1)]);
    assert_eq!(term.as_charlist_string(), None);
}

#[test]
fn test_as_list_or_empty_list() {
    let list = OwnedTerm::List(vec![OwnedTerm::Integer(1), OwnedTerm::Integer(2)]);
    assert_eq!(list.as_list_or_empty().len(), 2);
}

#[test]
fn test_as_list_or_empty_empty_list() {
    let list = OwnedTerm::List(vec![]);
    assert!(list.as_list_or_empty().is_empty());
}

#[test]
fn test_as_list_or_empty_non_list() {
    assert!(OwnedTerm::Integer(42).as_list_or_empty().is_empty());
    assert!(
        OwnedTerm::Atom(Atom::new("test"))
            .as_list_or_empty()
            .is_empty()
    );
    assert!(OwnedTerm::Nil.as_list_or_empty().is_empty());
}

#[test]
fn test_try_as_mfa() {
    let term = OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("erlang")),
        OwnedTerm::Atom(Atom::new("node")),
        OwnedTerm::Integer(0),
    ]);
    let mfa = term.try_as_mfa().unwrap();
    assert_eq!(mfa.module, Atom::new("erlang"));
    assert_eq!(mfa.function, Atom::new("node"));
    assert_eq!(mfa.arity, 0);
}

#[test]
fn test_try_as_mfa_invalid() {
    assert!(OwnedTerm::Integer(42).try_as_mfa().is_none());
}

#[test]
fn test_format_as_mfa() {
    let term = OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("lists")),
        OwnedTerm::Atom(Atom::new("map")),
        OwnedTerm::Integer(2),
    ]);
    assert_eq!(term.format_as_mfa(), Some("lists:map/2".to_string()));
}

#[test]
fn test_format_as_mfa_invalid() {
    assert_eq!(OwnedTerm::Integer(42).format_as_mfa(), None);
}

#[test]
fn test_format_as_pid() {
    let pid = ExternalPid::new(Atom::new("node@host"), 123, 456, 0);
    let term = OwnedTerm::Pid(pid);
    assert_eq!(term.format_as_pid(), Some("<123.456.0>".to_string()));
}

#[test]
fn test_format_as_pid_non_pid() {
    assert_eq!(OwnedTerm::Integer(42).format_as_pid(), None);
}

#[test]
fn test_proplist_get_atom_string() {
    let proplist = OwnedTerm::List(vec![OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("status")),
        OwnedTerm::Atom(Atom::new("running")),
    ])]);
    assert_eq!(
        proplist.proplist_get_atom_string("status"),
        Some("running".to_string())
    );
    assert_eq!(proplist.proplist_get_atom_string("missing"), None);
}

#[test]
fn test_proplist_get_atom_string_or() {
    let proplist = OwnedTerm::List(vec![OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("status")),
        OwnedTerm::Atom(Atom::new("running")),
    ])]);
    assert_eq!(
        proplist.proplist_get_atom_string_or("status", "unknown"),
        "running".to_string()
    );
    assert_eq!(
        proplist.proplist_get_atom_string_or("missing", "unknown"),
        "unknown".to_string()
    );
}

#[test]
fn test_proplist_get_pid_string() {
    let pid = ExternalPid::new(Atom::new("node@host"), 100, 200, 0);
    let proplist = OwnedTerm::List(vec![OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("group_leader")),
        OwnedTerm::Pid(pid),
    ])]);
    assert_eq!(
        proplist.proplist_get_pid_string("group_leader"),
        Some("<100.200.0>".to_string())
    );
    assert_eq!(proplist.proplist_get_pid_string("missing"), None);
}

#[test]
fn test_proplist_get_mfa_string() {
    let proplist = OwnedTerm::List(vec![OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("initial_call")),
        OwnedTerm::Tuple(vec![
            OwnedTerm::Atom(Atom::new("gen_server")),
            OwnedTerm::Atom(Atom::new("init_it")),
            OwnedTerm::Integer(6),
        ]),
    ])]);
    assert_eq!(
        proplist.proplist_get_mfa_string("initial_call"),
        Some("gen_server:init_it/6".to_string())
    );
    assert_eq!(proplist.proplist_get_mfa_string("missing"), None);
}

#[test]
fn test_proplist_get_mfa_string_or() {
    let proplist = OwnedTerm::List(vec![OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("current_function")),
        OwnedTerm::Tuple(vec![
            OwnedTerm::Atom(Atom::new("erlang")),
            OwnedTerm::Atom(Atom::new("hibernate")),
            OwnedTerm::Integer(3),
        ]),
    ])]);
    assert_eq!(
        proplist.proplist_get_mfa_string_or("current_function", "unknown"),
        "erlang:hibernate/3".to_string()
    );
    assert_eq!(
        proplist.proplist_get_mfa_string_or("missing", "unknown"),
        "unknown".to_string()
    );
}

#[test]
fn test_proplist_get_i64_or() {
    let proplist = OwnedTerm::List(vec![OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("memory")),
        OwnedTerm::Integer(1024),
    ])]);
    assert_eq!(proplist.proplist_get_i64_or("memory", 0), 1024);
    assert_eq!(proplist.proplist_get_i64_or("missing", 0), 0);
}

#[test]
fn test_proplist_get_bool_or() {
    let proplist = OwnedTerm::List(vec![OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("trap_exit")),
        OwnedTerm::Atom(Atom::new("true")),
    ])]);
    assert!(proplist.proplist_get_bool_or("trap_exit", false));
    assert!(!proplist.proplist_get_bool_or("missing", false));
}

#[test]
fn test_proplist_get_string_or() {
    let proplist = OwnedTerm::List(vec![OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("name")),
        OwnedTerm::List(vec![
            OwnedTerm::Integer(65),
            OwnedTerm::Integer(108),
            OwnedTerm::Integer(105),
            OwnedTerm::Integer(99),
            OwnedTerm::Integer(101),
        ]),
    ])]);
    assert_eq!(
        proplist.proplist_get_string_or("name", "unknown"),
        "Alice".to_string()
    );
    assert_eq!(
        proplist.proplist_get_string_or("missing", "unknown"),
        "unknown".to_string()
    );
}

#[test]
fn test_as_erlang_string_or() {
    let term = OwnedTerm::charlist("hello");
    assert_eq!(term.as_erlang_string_or("default"), "hello".to_string());
    assert_eq!(
        OwnedTerm::Integer(42).as_erlang_string_or("default"),
        "default".to_string()
    );
}

#[test]
fn test_as_erlang_string_or_binary() {
    let term = OwnedTerm::Binary(b"world".to_vec());
    assert_eq!(term.as_erlang_string_or("default"), "world".to_string());
}

#[test]
fn test_tuple_get() {
    let tuple = OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("ok")),
        OwnedTerm::Integer(42),
    ]);
    assert_eq!(tuple.tuple_get(0), Some(&OwnedTerm::Atom(Atom::new("ok"))));
    assert_eq!(tuple.tuple_get(1), Some(&OwnedTerm::Integer(42)));
    assert_eq!(tuple.tuple_get(2), None);
}

#[test]
fn test_tuple_get_on_non_tuple() {
    assert_eq!(OwnedTerm::Integer(42).tuple_get(0), None);
    assert_eq!(OwnedTerm::List(vec![]).tuple_get(0), None);
}

#[test]
fn test_tuple_get_string() {
    let tuple = OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("app")),
        OwnedTerm::charlist("description"),
        OwnedTerm::Binary(b"1.0.0".to_vec()),
    ]);
    assert_eq!(tuple.tuple_get_string(1), Some("description".to_string()));
    assert_eq!(tuple.tuple_get_string(2), Some("1.0.0".to_string()));
}

#[test]
fn test_tuple_get_string_or() {
    let tuple = OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("app")),
        OwnedTerm::charlist("description"),
    ]);
    assert_eq!(
        tuple.tuple_get_string_or(1, "unknown"),
        "description".to_string()
    );
    assert_eq!(
        tuple.tuple_get_string_or(5, "unknown"),
        "unknown".to_string()
    );
}

#[test]
fn test_tuple_get_atom_string() {
    let tuple = OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("kernel")),
        OwnedTerm::Integer(42),
    ]);
    assert_eq!(tuple.tuple_get_atom_string(0), Some("kernel".to_string()));
    assert_eq!(tuple.tuple_get_atom_string(1), None);
}

#[test]
fn test_tuple_get_atom_string_or() {
    let tuple = OwnedTerm::Tuple(vec![OwnedTerm::Atom(Atom::new("stdlib"))]);
    assert_eq!(
        tuple.tuple_get_atom_string_or(0, "unknown"),
        "stdlib".to_string()
    );
    assert_eq!(
        tuple.tuple_get_atom_string_or(1, "unknown"),
        "unknown".to_string()
    );
}

#[test]
fn test_tuple_get_combined_for_app_info() {
    let app_tuple = OwnedTerm::Tuple(vec![
        OwnedTerm::Atom(Atom::new("kernel")),
        OwnedTerm::charlist("ERTS  CXC 138 10"),
        OwnedTerm::charlist("9.0"),
    ]);
    assert_eq!(
        app_tuple.tuple_get_atom_string_or(0, "unknown"),
        "kernel".to_string()
    );
    assert_eq!(
        app_tuple.tuple_get_string_or(1, ""),
        "ERTS  CXC 138 10".to_string()
    );
    assert_eq!(app_tuple.tuple_get_string_or(2, ""), "9.0".to_string());
}

#[test]
fn test_erl_atom_macro() {
    let term = erl_atom!("hello");
    assert_eq!(term, OwnedTerm::Atom(Atom::new("hello")));
}

#[test]
fn test_erl_atoms_macro() {
    let term = erl_atoms!["a", "b", "c"];
    assert_eq!(
        term,
        OwnedTerm::List(vec![
            OwnedTerm::Atom(Atom::new("a")),
            OwnedTerm::Atom(Atom::new("b")),
            OwnedTerm::Atom(Atom::new("c")),
        ])
    );
}

#[test]
fn test_erl_atoms_macro_empty() {
    let term = erl_atoms![];
    assert_eq!(term, OwnedTerm::List(vec![]));
}

#[test]
fn test_erl_int_macro() {
    let term = erl_int!(42);
    assert_eq!(term, OwnedTerm::Integer(42));
}

#[test]
fn test_erl_int_macro_negative() {
    let term = erl_int!(-100);
    assert_eq!(term, OwnedTerm::Integer(-100));
}

#[test]
fn test_erl_tuple_macro() {
    let term = erl_tuple!(OwnedTerm::atom("ok"), OwnedTerm::Integer(42));
    assert_eq!(
        term,
        OwnedTerm::Tuple(vec![
            OwnedTerm::Atom(Atom::new("ok")),
            OwnedTerm::Integer(42),
        ])
    );
}

#[test]
fn test_erl_tuple_macro_empty() {
    let term = erl_tuple!();
    assert_eq!(term, OwnedTerm::Tuple(vec![]));
}

#[test]
fn test_erl_list_macro() {
    let term = erl_list!(
        OwnedTerm::Integer(1),
        OwnedTerm::Integer(2),
        OwnedTerm::Integer(3)
    );
    assert_eq!(
        term,
        OwnedTerm::List(vec![
            OwnedTerm::Integer(1),
            OwnedTerm::Integer(2),
            OwnedTerm::Integer(3),
        ])
    );
}

#[test]
fn test_erl_list_macro_empty() {
    let term = erl_list!();
    assert_eq!(term, OwnedTerm::List(vec![]));
}

#[test]
fn test_erl_map_macro() {
    let term = erl_map!(OwnedTerm::atom("key") => OwnedTerm::Integer(42));
    let mut expected = BTreeMap::new();
    expected.insert(OwnedTerm::Atom(Atom::new("key")), OwnedTerm::Integer(42));
    assert_eq!(term, OwnedTerm::Map(expected));
}

#[test]
fn test_erl_map_macro_empty() {
    let term = erl_map!();
    assert_eq!(term, OwnedTerm::Map(BTreeMap::new()));
}

#[test]
fn test_erl_macros_combined() {
    let term = erl_tuple!(
        erl_atom!("reply"),
        erl_list!(erl_int!(1), erl_int!(2)),
        erl_atoms!["a", "b"]
    );
    assert_eq!(
        term,
        OwnedTerm::Tuple(vec![
            OwnedTerm::Atom(Atom::new("reply")),
            OwnedTerm::List(vec![OwnedTerm::Integer(1), OwnedTerm::Integer(2)]),
            OwnedTerm::List(vec![
                OwnedTerm::Atom(Atom::new("a")),
                OwnedTerm::Atom(Atom::new("b")),
            ]),
        ])
    );
}

#[test]
fn test_is_charlist_rejects_surrogates() {
    let term = OwnedTerm::List(vec![OwnedTerm::Integer(0xD800)]);
    assert!(!term.is_charlist());

    let term = OwnedTerm::List(vec![OwnedTerm::Integer(0xDFFF)]);
    assert!(!term.is_charlist());

    let term = OwnedTerm::List(vec![OwnedTerm::Integer(0xD7FF)]);
    assert!(term.is_charlist());

    let term = OwnedTerm::List(vec![OwnedTerm::Integer(0xE000)]);
    assert!(term.is_charlist());
}
