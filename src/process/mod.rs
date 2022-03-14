use std::time::Duration;

use crate::{host_api, module::WasmModule, LunaticError, ProcessConfig, Tag};

mod background_task;
mod gen_server;
mod macros;
mod proc;
mod protocol;
mod server;
mod supervisor;
mod task;

/// `IntoProcess` is a helper trait to generalize over the [`spawn`] function.
///
/// The `Handler` is usually a function that represents the entry point to the process or handles
/// individual messages. Some types, like [`GenericServer`], already define a variety of handlers
///  bound to the type and use this associated type to provide an `init` function.
///
/// The generic parameter `C` allows spawned processes to transfer some state to the newly spawned
/// process. It's usually used together in combination with the `Handler` type to define a function
/// signature that receives the transferred state as an argument.
pub trait IntoProcess<C> {
    // The type of the 2nd argument passed to the [`spawn`] function.
    type Handler;
    // Spawn's a new process and returns a handle to it.
    fn spawn(
        module: Option<WasmModule>,
        config: Option<&ProcessConfig>,
        capture: C,
        handler: Self::Handler,
    ) -> Result<Self, LunaticError>
    where
        Self: Sized;
}

/// Spawn a new process.
///
/// There are multiple kinds of processes you can spawn:
///
/// * [`Process`] - A process that can receive messages through a [`Mailbox`](crate::Mailbox).
/// * [`Task`] - One-off process that returns a value.
/// * [`BackgroundTask`] - One-off process that doesn't return a value.
/// * [`Server`] - Abstracts the common client-server interaction and can handle requests of the
///                same type.
/// * [`GenericServer`] - Abstracts the common client-server interaction and can handle requests
///                       of different types.
/// * [`Supervisor`] - A process that can supervise others and re-spawn them if they trap.
///
/// Refer to their individual documentation to see how they interact with the `spawn` function.
pub fn spawn<T, C>(capture: C, handler: T::Handler) -> Result<T, LunaticError>
where
    T: IntoProcess<C>,
{
    <T as IntoProcess<C>>::spawn(None, None, capture, handler)
}

pub fn spawn_config<T, C>(
    config: &ProcessConfig,
    capture: C,
    handler: T::Handler,
) -> Result<T, LunaticError>
where
    T: IntoProcess<C>,
{
    <T as IntoProcess<C>>::spawn(None, Some(config), capture, handler)
}

/// `IntoProcessLink` is a helper trait to generalize over the [`spawn_link`] function.
///
/// The `Handler` is usually a function that represents the entry point to the process or handles
/// individual messages. Some types, like [`GenericServer`], already define a variety of handlers
///  bound to the type and use this associated type to provide an `init` function.
///
/// The generic parameter `C` allows spawned processes to transfer some state to the newly spawned
/// process. It's usually used together in combination with the `Handler` type to define a function
/// signature that receives the transferred state as an argument.
pub trait IntoProcessLink<C> {
    // The type of the 2nd argument passed to the [`spawn`] function.
    type Handler;
    // Spawn's a new process and returns a handle to it.
    fn spawn_link(
        module: Option<WasmModule>,
        config: Option<&ProcessConfig>,
        tag: Tag,
        capture: C,
        handler: Self::Handler,
    ) -> Result<Self, LunaticError>
    where
        Self: Sized;
}

/// Spawn a new process and link it to the caller.
//
// TODO: Research if `spawn` and `spawn_link` could move the whole spawning procedure into the new
//       async task, so that there can't be any failure during the host call and we can return `T`
//       instead of a `Result` here.
pub fn spawn_link<T, C>(capture: C, handler: T::Handler) -> Result<T, LunaticError>
where
    T: IntoProcessLink<C>,
{
    <T as IntoProcessLink<C>>::spawn_link(None, None, Tag::new(), capture, handler)
}

pub fn spawn_link_config<T, C>(
    config: &ProcessConfig,
    capture: C,
    handler: T::Handler,
) -> Result<T, LunaticError>
where
    T: IntoProcessLink<C>,
{
    <T as IntoProcessLink<C>>::spawn_link(None, Some(config), Tag::new(), capture, handler)
}

/// Suspends the current process for `duration` of time.
pub fn sleep(duration: Duration) {
    unsafe { host_api::process::sleep_ms(duration.as_millis() as u64) };
}

// re-export all process types
pub use background_task::BackgroundTask;
pub use gen_server::{GenericServer, HandleMessage, HandleRequest};
pub use macros::*;
pub use proc::Process;
pub use protocol::{session::*, Protocol};
pub use server::Server;
pub use supervisor::{HandleSupervisorMessage, HandleSupervisorRequest, Supervise, Supervisor};
pub use task::Task;
