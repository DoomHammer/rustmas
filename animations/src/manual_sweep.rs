use animation_api::parameter_schema::{EnumOption, Parameter, ParameterValue, ParametersSchema};
use animation_api::Animation;
use animation_utils::decorators::BrightnessControlled;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Serialize, Deserialize)]
enum Alignment {
    Before,
    Center,
    After,
}

#[derive(Serialize, Deserialize)]
struct Parameters {
    axis: Axis,
    band_alignment: Alignment,
    band_size: f64,
    band_position: f64,
    color: lightfx::Color,
}

#[animation_utils::plugin]
pub struct ManualSweep {
    points: Vec<(f64, f64, f64)>,
    parameters: Parameters,
}

impl ManualSweep {
    pub fn create(points: Vec<(f64, f64, f64)>) -> impl Animation {
        BrightnessControlled::new(Self {
            points,
            parameters: Parameters {
                axis: Axis::Y,
                band_alignment: Alignment::Center,
                band_size: 0.1,
                band_position: 0.0,
                color: lightfx::Color::white(),
            },
        })
    }
}

impl Animation for ManualSweep {
    fn update(&mut self, _delta: f64) {}

    fn render(&self) -> lightfx::Frame {
        self.points
            .iter()
            .map(|(x, y, z)| match self.parameters.axis {
                Axis::X => *x,
                Axis::Y => *y,
                Axis::Z => *z,
            })
            .map(|h| {
                let (start, end) = match self.parameters.band_alignment {
                    Alignment::Before => (
                        self.parameters.band_position - self.parameters.band_size,
                        self.parameters.band_position,
                    ),
                    Alignment::Center => (
                        self.parameters.band_position - self.parameters.band_size / 2.0,
                        self.parameters.band_position + self.parameters.band_size / 2.0,
                    ),
                    Alignment::After => (
                        self.parameters.band_position,
                        self.parameters.band_position + self.parameters.band_size,
                    ),
                };
                if h > start && h < end {
                    self.parameters.color
                } else {
                    lightfx::Color::black()
                }
            })
            .into()
    }

    fn animation_name(&self) -> &str {
        "Testing: Manual sweep"
    }

    fn get_fps(&self) -> f64 {
        0.0
    }

    fn parameter_schema(&self) -> ParametersSchema {
        ParametersSchema {
            parameters: vec![
                Parameter {
                    id: "axis".to_owned(),
                    name: "Direction".to_owned(),
                    description: Some("Direction of the sweep".to_owned()),
                    value: ParameterValue::Enum {
                        values: vec![
                            EnumOption {
                                name: "X: Left-Right".to_owned(),
                                description: None,
                                value: "X".to_owned(),
                            },
                            EnumOption {
                                name: "Y: Bottom-Top".to_owned(),
                                description: None,
                                value: "Y".to_owned(),
                            },
                            EnumOption {
                                name: "Z: Front-Back".to_owned(),
                                description: None,
                                value: "Z".to_owned(),
                            },
                        ],
                    },
                },
                Parameter {
                    id: "band_position".to_owned(),
                    name: "Band position".to_owned(),
                    description: Some(
                        "Position of the start (left, bottom, front) of the band".to_owned(),
                    ),
                    value: ParameterValue::Number {
                        min: -1.0,
                        max: 1.0,
                        step: 0.01,
                    },
                },
                Parameter {
                    id: "band_size".to_owned(),
                    name: "Band size".to_owned(),
                    description: Some("Thickness of the sweep band".to_owned()),
                    value: ParameterValue::Number {
                        min: 0.0,
                        max: 2.0,
                        step: 0.01,
                    },
                },
                Parameter {
                    id: "band_alignment".to_owned(),
                    name: "Band alignment".to_owned(),
                    description: None,
                    value: ParameterValue::Enum {
                        values: vec![
                            EnumOption {
                                name: "Before".into(),
                                description: None,
                                value: "Before".into(),
                            },
                            EnumOption {
                                name: "Center".into(),
                                description: None,
                                value: "Center".into(),
                            },
                            EnumOption {
                                name: "After".into(),
                                description: None,
                                value: "After".into(),
                            },
                        ],
                    },
                },
                Parameter {
                    id: "color".to_owned(),
                    name: "Color".to_owned(),
                    description: Some("Color of the sweep band".to_owned()),
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
