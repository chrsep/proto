use tetra::graphics::Rectangle;

pub trait Collidable {
    fn get_collision_box(self) -> Rectangle;
    fn check_collision(self, collision_box: Rectangle) -> bool;
}

pub fn going_to_collide_top<F, S>(first: F, second: S) -> bool
where
    F: Collidable,
    S: Collidable,
{
    let mut second_collision_box = second.get_collision_box();
    // pull collision box upwards so it triggers before everything else
    second_collision_box.y -= 1.0;
    // reduce collision box width to prevent from being triggered by left touch
    second_collision_box.x += 10.0;
    // reduce collision box width to prevent from being triggered by right touch
    second_collision_box.width -= 20.0;
    // reduce collision box height to prevent from being triggered by bottom touch
    second_collision_box.height = 1.0;
    first.check_collision(second_collision_box)
}

pub fn going_to_collide_bottom<F, S>(first: F, second: S) -> bool
where
    F: Collidable,
    S: Collidable,
{
    let mut second_collision_box = second.get_collision_box();
    // push collision box downwards so it triggers before everything else
    second_collision_box.y += second_collision_box.height;
    // reduce collision box width to prevent from being triggered by left touch
    second_collision_box.x += 10.0;
    // reduce collision box width to prevent from being triggered by right touch
    second_collision_box.width -= 20.0;
    // reduce collision height to prevent from being triggered by top touch
    second_collision_box.height = 1.0;
    first.check_collision(second_collision_box)
}

pub fn going_to_collide_left<F, S>(first: F, second: S) -> bool
where
    F: Collidable,
    S: Collidable,
{
    let mut second_collision_box = second.get_collision_box();
    second_collision_box.x -= 1.0;
    // reduce sensitivity to other sides
    second_collision_box.y += 10.0;
    second_collision_box.height -= 20.0;
    second_collision_box.width = 1.0;
    first.check_collision(second_collision_box)
}

pub fn going_to_collide_right<F, S>(first: F, second: S) -> bool
where
    F: Collidable,
    S: Collidable,
{
    let mut second_collision_box = second.get_collision_box();
    second_collision_box.x += second_collision_box.width;

    // reduce sensitivity to other sides
    second_collision_box.y += 10.0;
    second_collision_box.height -= 20.0;
    second_collision_box.width = 1.0;
    first.check_collision(second_collision_box)
}
