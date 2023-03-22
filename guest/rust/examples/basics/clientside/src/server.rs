use ambient_api::{
    components::core::{primitives::cube, rendering::color, transform::translation},
    concepts::make_transformable,
    message::server as message,
    prelude::*,
};

use crate::components::{grid_side_length, grid_x, grid_y};

#[main]
pub async fn main() -> EventResult {
    let side_length = 10;
    entity::add_component(
        entity::synchronized_resources(),
        grid_side_length(),
        side_length,
    );

    for y in 0..2 * side_length + 1 {
        for x in 0..2 * side_length + 1 {
            Entity::new()
                .with_merge(make_transformable())
                .with_default(cube())
                .with(grid_x(), x)
                .with(grid_y(), y)
                .with(color(), Vec4::ONE)
                .with(
                    translation(),
                    vec3((x - side_length) as f32, (y - side_length) as f32, 0.0),
                )
                .spawn();
        }
    }

    message::subscribe_bytes("test", |source, data| {
        println!("{source:?}");
        println!("{:?}", String::from_utf8(data));

        if let message::Source::Network { user_id } = source {
            message::send(
                message::Target::NetworkTargetedReliable {
                    user_id: user_id.clone(),
                },
                "test",
                String::from("Hello, world from the server (unistream)!").into_bytes(),
            );

            message::send(
                message::Target::NetworkTargetedUnreliable {
                    user_id: user_id.clone(),
                },
                "test",
                String::from("Hello, world from the server (datagram)!").into_bytes(),
            );
        }

        EventOk
    });

    EventOk
}
