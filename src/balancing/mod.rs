mod frontend;
mod middleware;

pub use self::{
    frontend::Server,
    middleware::{
        adapters::{
            chain::chain,
            proj::{fst, snd},
        },
        client::connect,
        param::param,
        route::route,
        selector::cyclical::RoundRobin,
        splice::splice,
        PipeBuilder,
    },
};
