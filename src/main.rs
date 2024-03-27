use hs_hackathon::prelude::*;

async fn motion_step(
    wheels: &mut WheelOrientation,
    motor: &mut MotorSocket,
    bearing_to_target: f64,
    distance_to_target: f64,
) {
    const ANGLE_THRESHOLD : f64 = 10.0;

    let forwards = if distance_to_target < ANGLE_THRESHOLD {
        println!("stop");
        Velocity::none()
    } else {
        println!("fwd");
        Velocity::forward()
    };

    let direction = if bearing_to_target >= ANGLE_THRESHOLD {
        println!("right");
        Angle::right()
    } else if bearing_to_target <= -10.0 {
        println!("left");
        Angle::left()
    } else {
        println!("straight");
        Angle::straight()
    };

    let ok = wheels.set(direction).await;
    println!("steering: {:?}", ok);

    let ok = motor.move_for(forwards, std::time::Duration::from_secs(10)).await;
    println!("motor: {:?}", ok);
}

fn bearing_to_target(vehicle_bearing: f64, target_bearing: f64) -> f64 {
    vehicle_bearing - target_bearing
}

#[hs_hackathon::main]
async fn main() -> eyre::Result<()> {
    let mut wheels = WheelOrientation::new().await?;
    let mut motor = MotorSocket::open().await?;
//    let mut drone = Camera::connect().await?;

    // Does this stop the motor driver being fucky?
    // (spoiler: no)
    motor.move_for(Velocity::forward(), std::time::Duration::from_secs(1)).await;

    motion_step(&mut wheels, &mut motor, bearing_to_target(15.0, 30.0), 50.0).await;

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    motion_step(&mut wheels, &mut motor, bearing_to_target(15.0, 2.0), 40.0).await;

    Ok(())
}
