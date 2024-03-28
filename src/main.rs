use hs_hackathon::prelude::*;

#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl From<BoundingBox> for Point {
    fn from(b: BoundingBox) -> Self {
        let x = (b.x_min() as f64 + b.x_max() as f64) / 2.0;
        let y = (b.y_min() as f64 + b.y_max() as f64) / 2.0;
        Self { x, y }
    }
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> f64 {
        let x = (self.x - other.x).abs().powi(2);
        let y = (self.y - other.y).abs().powi(2);
        (x + y).sqrt()
    }

    pub fn bearing_to(&self, other: &Self) -> f64 {
        if self == other {
            return 0.0;
        }

        let dx = other.x - self.x;
        let dy = other.y - self.y;

        let radians = dy.atan2(dx);
        let degrees = radians.to_degrees();

        let degrees = -degrees + 90.0;

        if degrees > 180.0 {
            degrees - 360.0
        } else {
            degrees
        }
    }
}



struct VisionState {
    target_position: Point,
    car_position: Point,
}

impl VisionState {
    fn get_distance_to_target(&self) -> f64 {
        self.car_position.distance_to(&self.target_position)
    }
}

fn command_from_vision_state(current: &VisionState, old: &VisionState) -> (f64, f64) {
    let car_bearing = old.car_position.bearing_to(&current.car_position);
    println!("car bearing: {car_bearing}");

    let car_desired_bearing = current.car_position.bearing_to(&current.target_position);
    println!("car desired bearing: {car_desired_bearing}");

    let car_bearing_to_target = car_desired_bearing - car_bearing;
    println!("car bearing to target: {car_bearing_to_target}");

    let car_distance_to_target = current.get_distance_to_target();
    println!("car distance to target: {car_distance_to_target}");

    (car_bearing_to_target, car_distance_to_target)
}

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
    let mut drone = Camera::connect().await?;
    
    let ledconfig = LedDetectionConfig{
        threshold_value: 20,
        ..Default::default()
    };
    
    // let old_state = todo!();
    while let Ok(frame) = drone.snapshot().await.map_err(|err| eprintln!("{err}")) {
        // 1. Process  frame into 2 points 
        
        match detect(&frame.0, &ledconfig){
                Ok(leds) => {
                    println!("Leds : {:#?}",leds);

                    let car = leds.iter().find(|l| l.color == Color::Red);
                    let target = leds.iter().find(|l| l.color == Color::Green || l.color == Color::White);
                    println!("car: {:#?}", car);
                    println!("target: {:#?}", target);

                    if let (Some(car), Some(target)) = (car, target) {
                        let car_pos : Point = car.bbox.into();
                        let target_pos : Point = target.bbox.into();
                        println!("car: {:#?}", car_pos);
                        println!("target: {:#?}", target_pos);
                    }

                },
                Err(err) => {
                    println!("Report : {}" , err);
                },
        };

//        let current_state = todo!();

        // 2. Naviagate vehicle based on two points
         
        // let (bearing, distance) = command_from_vision_state(current_state, old_state);
        // motion_step(&mut wheels, &mut motor, bearing, distance).await;

        // old_state = current_state;

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        // TODO exit when reach goal
    }

    Ok(())
}
