use hs_hackathon::prelude::*;

async fn motion_step(
    wheels: &mut WheelOrientation,
    motor: &mut MotorSocket,
    bearing_to_target: f64,
    distance_to_target: f64,
) {
    const DISTANCE_THRESHOLD: f64 = 10.0;
    const ANGLE_THRESHOLD: f64 = 10.0;

    let forwards = if distance_to_target < DISTANCE_THRESHOLD {
        println!("stop");
        Velocity::none()
    } else {
        println!("fwd");
        Velocity::forward()
    };

    let direction = if bearing_to_target >= ANGLE_THRESHOLD {
        println!("left");
        Angle::left()
    } else if bearing_to_target <= -ANGLE_THRESHOLD {
        println!("right");
        Angle::right()
    } else {
        println!("straight");
        Angle::straight()
    };

    let ok = wheels.set(direction).await;
    println!("steering: {:?}", ok);

    let ok = motor
        .move_for(forwards, std::time::Duration::from_secs(10))
        .await;
    println!("motor: {:?}", ok);
}

#[hs_hackathon::main]
async fn main() -> eyre::Result<()> {
    let mut wheels = WheelOrientation::new().await?;
    let mut motor = MotorSocket::open().await?;
    // let mut drone = Camera::connect().await?;

    // Demo
    motion_step(&mut wheels, &mut motor, 30.0, 50.0).await;
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    motion_step(&mut wheels, &mut motor, -12.0, 40.0).await;

    Ok(())
}
