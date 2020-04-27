use crate::{schedule::Schedulable, SystemBuilder};
use legion_core::{
    filter::EntityFilter,
    query::{DefaultFilter, IntoQuery, View},
};

pub fn into_system<'a, Q, F, R>(name: &'static str, system: F) -> Box<dyn Schedulable>
where
    Q: IntoQuery + DefaultFilter<Filter = R>,
    F: Fn(<<Q as View<'a>>::Iter as Iterator>::Item) + Send + Sync + 'static,
    R: EntityFilter + Sync + 'static,
{
    SystemBuilder::new(name)
        .with_query(Q::query())
        .build(move |_, world, _, query| {
            for x in query.iter_mut(world) {
                system(x);
            }
        })
}

#[cfg(test)]
mod tests {
    use super::into_system;
    use crate::resource::Resources;
    use legion_core::{
        borrow::{Ref, RefMut},
        world::World,
    };

    struct Y(usize);
    struct X(usize);

    fn read_write_system((x, mut y): (Ref<X>, RefMut<Y>)) {
        y.0 += 1;
        println!("{} {}", x.0, y.0);
    }

    fn read_system(x: Ref<X>) {
        println!("{}", x.0);
    }

    #[test]
    fn test_system() {
        let mut world = World::new();
        let mut resources = Resources::default();
        {
            world.insert((), vec![(X(1), Y(1)), (X(2), Y(2))]);
        }
        // This works
        let mut system = into_system::<(Ref<_>, RefMut<_>), _, _>("read_write", read_write_system);
        system.run(&mut world, &mut resources);
        
        // This doesn't work
        // let mut x = into_system("simple", read_system);
        // x.run(&mut world, &mut resources);

        let mut x_read_system = into_system::<Ref<_>, _, _>("read", read_system);
        x_read_system.run(&mut world, &mut resources);
    }
}