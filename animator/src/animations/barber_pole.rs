use super::{utils, Animation};
use lightfx::schema::{Parameter, ParameterValue, ParametersSchema};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Parameters {
    color_a: lightfx::Color,
    color_b: lightfx::Color,
}

pub struct BarberPole {
    points_polar: Vec<(f64, f64, f64)>,
    parameters: Parameters,
}

impl BarberPole {
    pub fn new(points: &Vec<(f64, f64, f64)>) -> Self {
        Self {
            points_polar: points.iter().map(utils::to_polar).collect(),
            parameters: Parameters {
                color_a: lightfx::Color::rgb(255, 0, 0),
                color_b: lightfx::Color::rgb(255, 255, 255),
            },
        }
    }
}

impl Animation for BarberPole {
    fn frame(&mut self, time: f64) -> lightfx::Frame {
        self.points_polar
            .iter()
            .map(|(_, a, h)| {
                if (a / (std::f64::consts::PI * 2.0) + h + time / 2.0) % 0.5 - 0.25 < 0.0 {
                    self.parameters.color_a
                } else {
                    self.parameters.color_b
                }
            })
            .into()
    }

    fn parameter_schema(&self) -> ParametersSchema {
        ParametersSchema {
            parameters: vec![
                Parameter {
                    id: "color_a".to_owned(),
                    name: "First color".to_owned(),
                    description: None,
                    value: ParameterValue::Color,
                },
                Parameter {
                    id: "color_b".to_owned(),
                    name: "Second color".to_owned(),
                    description: None,
                    value: ParameterValue::Color,
                },
            ],
        }
    }

    fn set_parameters(
        &mut self,
        parameters: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.parameters = serde_json::from_value(parameters)?;
        Ok(())
    }

    fn get_parameters(&self) -> serde_json::Value {
        json!(self.parameters)
    }
}
