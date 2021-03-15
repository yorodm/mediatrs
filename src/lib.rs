use std::any::{Any, TypeId};
use std::collections::HashMap;

pub trait Request: 'static {}
pub trait Handler<I, O>
where
    I: Request,
    O: Sized,
{
    fn handle<'a, 'b>(&'a self, r: &'a I) -> O;
}

pub(crate) struct TypeMap(HashMap<TypeId, Box<dyn Any>>);

impl TypeMap {
    pub fn new() -> TypeMap {
        TypeMap(HashMap::<TypeId, Box<dyn Any>>::new())
    }

    pub fn set<R: 'static, H: Any + 'static>(&mut self, t: H) {
        println!("insertando {:?}", TypeId::of::<R>());
        self.0.insert(TypeId::of::<R>(), Box::new(t));
    }

    pub fn get_mut<R: 'static, H: 'static + Any>(&mut self) -> Option<&mut H> {
        println!("Pidiendo {:?}", TypeId::of::<R>());
        self.0
            .get_mut(&TypeId::of::<R>())
            .and_then(|t| t.downcast_mut::<H>())
    }
}

pub struct Mediator(TypeMap);

type Wrapper<R, T> = Box<dyn Handler<R, T>>;

impl Mediator {
    pub fn new() -> Mediator {
        Mediator(TypeMap::new())
    }
    pub fn add_handler<R, H, T>(&mut self, f: H)
    where
        R: Request,
        H: Handler<R, T> + 'static,
        T: 'static,
    {
        self.0.set::<R, Wrapper<R, T>>(Box::new(f));
    }

    pub fn send<R: Request, T: 'static>(&mut self, r: &R) -> Option<T> {
        self.0.get_mut::<R, Wrapper<R, T>>().map(|h| h.handle(r))
    }
}

#[cfg(test)]
mod test {
    use super::{Handler, Request,Mediator};
    pub struct Ping;
    pub struct PingHandler;
    impl Request for Ping {}
    impl Handler<Ping, String> for PingHandler {
        fn handle(&self, _: &Ping) -> String {
            "Pong".to_owned()
        }
    }
    #[test]
    fn test_ping_request() {
        let mut m = Mediator::new();
        m.add_handler::<Ping, _, _>(PingHandler {});
        let x: Option<String> = m.send::<Ping, _>(&Ping {});
        assert_eq!(x, Some("Pong".to_owned()))
    }
}
