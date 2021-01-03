use crate::parser;
use crate::utils;

const HEADER: &str = r#"
use fltk::browser::*;
use fltk::button::*;
use fltk::dialog::*;
use fltk::frame::*;
use fltk::group::*;
use fltk::image::*;
use fltk::input::*;
use fltk::menu::*;
use fltk::misc::*;
use fltk::output::*;
use fltk::prelude::*;
use fltk::table::*;
use fltk::text::*;
use fltk::tree::*;
use fltk::valuator::*;
use fltk::widget::*;
use fltk::window::*;"#;

pub fn generate(ast: &[parser::Token]) -> String {
    let mut s = "pub struct ".to_string();
    let mut ctor = "    Self { ".to_string();
    let mut imp = "impl ".to_string();
    for elem in ast {
        use parser::TokenType::*;
        match &elem.typ {
            Class => {
                s += &elem.ident;
                s += "{\n";
                imp += &elem.ident;
                imp += "{\n";
            }
            Function(_) => {
                imp += "    pub fn ";
                imp += &elem.ident;
                if !elem.ident.contains("-> Self") {
                    imp += "-> Self";
                }
                imp += " {\n";
            }
            Member(t, p, is_parent, props) => {
                if t != "MenuItem" {
                    s += &format!("    pub {}: {},\n", &elem.ident, t);
                    ctor += &elem.ident;
                    ctor += ", ";
                }
                let xywh = props.iter().position(|x| x == "xywh").unwrap();
                let label = props.iter().position(|x| x == "label");
                if !is_parent {
                    if t != "MenuItem" {
                        imp += &format!(
                            "\tlet mut {} = {}::new({}, \"{}\");\n",
                            &elem.ident,
                            &t,
                            utils::unbracket(&props[xywh + 1].replace(" ", ", ")),
                            if let Some(l) = label {
                                utils::unbracket(&props[l + 1])
                            } else {
                                ""
                            }
                        );
                    }
                } else {
                    imp += &format!(
                        "\tlet mut {0} = {1}::new({2}, \"{3}\");\n\t{0}.end();\n",
                        &elem.ident,
                        &t,
                        utils::unbracket(&props[xywh + 1].replace(" ", ", ")),
                        if let Some(l) = label {
                            utils::unbracket(&props[l + 1])
                        } else {
                            ""
                        }
                    );
                }
                for i in 0..props.len() {
                    match props[i].as_str() {
                        "visible" => {
                            imp += &format!("\t{}.show();\n", &elem.ident,);
                        }
                        "color" => {
                            imp += &format!(
                                "\t{}.set_color(Color::from_u32({}));\n",
                                &elem.ident,
                                utils::unbracket(&props[i + 1])
                            );
                        }
                        "selection_color" => {
                            imp += &format!(
                                "\t{}.set_selection_color({});\n",
                                &elem.ident,
                                utils::global_to_pascal(utils::unbracket(&props[i + 1]))
                            );
                        }
                        "labelsize" => {
                            imp += &format!(
                                "\t{}.set_label_size({});\n",
                                &elem.ident,
                                utils::unbracket(&props[i + 1])
                            );
                        }
                        "textsize" => {
                            imp += &format!(
                                "\t{}.set_text_size({});\n",
                                &elem.ident,
                                utils::unbracket(&props[i + 1])
                            );
                        }
                        "labeltype" => {
                            imp += &format!(
                                "\t{}.set_label_type(LabelType::{});\n",
                                &elem.ident,
                                utils::global_to_pascal(utils::unbracket(&props[i + 1]))
                            );
                        }
                        "labelcolor" => {
                            imp += &format!(
                                "\t{}.set_label_color({});\n",
                                &elem.ident,
                                utils::unbracket(&props[i + 1])
                            );
                        }
                        "labelfont" => {
                            imp += &format!(
                                "\t{}.set_label_font(unsafe {{std::mem::transmute({})}});\n",
                                &elem.ident,
                                utils::unbracket(&props[i + 1])
                            );
                        }
                        "textfont" => {
                            imp += &format!(
                                "\t{}.set_text_font(unsafe {{std::mem::transmute({})}});\n",
                                &elem.ident,
                                utils::unbracket(&props[i + 1])
                            );
                        }
                        "box" => {
                            imp += &format!(
                                "\t{}.set_frame(FrameType::{});\n",
                                &elem.ident,
                                utils::global_to_pascal(utils::unbracket(&props[i + 1]))
                            );
                        }
                        // "down_box" => {
                        //     imp += &format!(
                        //         "\t{}.set_frame({});\n",
                        //         &elem.ident,
                        //         utils::unbracket(&props[i + 1])
                        //     );
                        // },
                        "when" => {
                            imp += &format!(
                                "\t{}.set_trigger(unsafe {{std::mem::transmute({})}});\n",
                                &elem.ident,
                                utils::unbracket(&props[i + 1])
                            );
                        }
                        "tooltip" => {
                            imp += &format!(
                                "\t{}.set_tooltip(\"{}\");\n",
                                &elem.ident,
                                utils::unbracket(&props[i + 1])
                            );
                        }
                        "maximum" => {
                            imp += &format!(
                                "\t{}.set_maximum({});\n",
                                &elem.ident,
                                utils::unbracket(&props[i + 1])
                            );
                        }
                        "minimum" => {
                            imp += &format!(
                                "\t{}.set_minimum({});\n",
                                &elem.ident,
                                utils::unbracket(&props[i + 1])
                            );
                        }
                        "step" => {
                            imp += &format!(
                                "\t{}.set_step({});\n",
                                &elem.ident,
                                utils::unbracket(&props[i + 1])
                            );
                        }
                        "value" => {
                            imp += &format!(
                                "\t{}.set_value(\"{}\");\n",
                                &elem.ident,
                                utils::unbracket(&props[i + 1])
                            );
                        }
                        "type" => {
                            if props[i + 1] != "Double" {
                                imp += &format!(
                                    "\t{}.set_type({}Type::{});\n",
                                    &elem.ident,
                                    t,
                                    utils::global_to_pascal(utils::unbracket(&props[i + 1]))
                                );
                            }
                        }
                        "align" => {
                            imp += &format!(
                                "\t{}.set_align(unsafe {{std::mem::transmute({})}});\n",
                                &elem.ident,
                                utils::unbracket(&props[i + 1])
                            );
                        }
                        "shortcut" => {
                            imp += &format!(
                                "\t{}.set_shortcut(unsafe {{std::mem::transmute({})}});\n",
                                &elem.ident,
                                utils::unbracket(&props[i + 1])
                            );
                        }
                        "image" => {
                            imp += &format!(
                                "\t{}.set_image(Some(SharedImage::load(\"{}\").unwrap()));\n",
                                &elem.ident,
                                utils::unbracket(&props[i + 1])
                            );
                        }
                        "hide" => {
                            imp += &format!("\t{}.hide();\n", &elem.ident,);
                        }
                        "modal" => {
                            imp += &format!("\t{}.make_modal(true);\n", &elem.ident,);
                        }
                        "resizable" => {
                            if *is_parent {
                                imp += &format!("\t{}.make_resizable(true);\n", &elem.ident,);
                            }
                        },
                        _ => (),
                    }
                }
                if let Some(parent) = p {
                    if !parent.is_empty() {
                        let parent = if parent[parent.len() - 1] != elem.ident {
                            parent[parent.len() - 1].clone()
                        } else {
                            if !parent[parent.len() - 2].contains("(") {
                                parent[parent.len() - 2].clone()
                            } else {
                                continue;
                            }
                        };
                        if t != "MenuItem" {
                            imp += &format!("\t{}.add(&{});\n", parent, &elem.ident);
                        } else {
                            imp += &format!(
                                "\t{}.add(\"{}\", Shortcut::None, MenuFlag::Normal, || {{}});\n",
                                parent,
                                if let Some(l) = label {
                                    utils::unbracket(&props[l + 1])
                                } else {
                                    ""
                                },
                            );
                        }
                    }
                }
            }
            _ => (),
        }
    }
    ctor += "}";
    format!("{}\n\n{}}}\n\n{}    {}\n    }}\n}}\n", HEADER, s, imp, ctor)
}
