//! Message handling for proto serialization and deserialization.

use platypus_core::element::ElementType;
use platypus_core::state::Delta as CoreDelta;
use platypus_proto::*;
use prost::Message;
use uuid::Uuid;

/// Convert Rust ElementType to proto Element
pub fn element_type_to_proto(id: u64, element: &ElementType) -> Element {
    let id_str = id.to_string();
    
    let element_type = match element {
        ElementType::Text { value } => {
            element::Type::Text(TextElement {
                value: value.clone(),
            })
        }
        ElementType::Markdown { value } => {
            element::Type::Markdown(MarkdownElement {
                value: value.clone(),
            })
        }
        ElementType::Code { value, language } => {
            element::Type::Code(CodeElement {
                value: value.clone(),
                language: language.clone().unwrap_or_default(),
            })
        }
        ElementType::Heading { value, level } => {
            element::Type::Heading(HeadingElement {
                value: value.clone(),
                level: *level,
            })
        }
        ElementType::Button { label, key } => {
            element::Type::Button(ButtonElement {
                label: label.clone(),
                key: key.clone().unwrap_or_default(),
            })
        }
        ElementType::TextInput { label, value, key } => {
            element::Type::TextInput(TextInputElement {
                label: label.clone(),
                value: value.clone(),
                key: key.clone().unwrap_or_default(),
            })
        }
        ElementType::TextArea { label, value, key } => {
            element::Type::TextArea(TextAreaElement {
                label: label.clone(),
                value: value.clone(),
                key: key.clone().unwrap_or_default(),
            })
        }
        ElementType::NumberInput { label, value, key } => {
            element::Type::NumberInput(NumberInputElement {
                label: label.clone(),
                value: *value,
                key: key.clone().unwrap_or_default(),
            })
        }
        ElementType::Slider {
            label,
            value,
            min,
            max,
            key,
        } => {
            element::Type::Slider(SliderElement {
                label: label.clone(),
                value: *value,
                min: *min,
                max: *max,
                key: key.clone().unwrap_or_default(),
            })
        }
        ElementType::Checkbox { label, value, key } => {
            element::Type::Checkbox(CheckboxElement {
                label: label.clone(),
                value: *value,
                key: key.clone().unwrap_or_default(),
            })
        }
        ElementType::Radio {
            label,
            options,
            value,
            key,
        } => {
            element::Type::Radio(RadioElement {
                label: label.clone(),
                options: options.clone(),
                value: value.clone().unwrap_or_default(),
                key: key.clone().unwrap_or_default(),
            })
        }
        ElementType::Selectbox {
            label,
            options,
            value,
            key,
        } => {
            element::Type::Selectbox(SelectboxElement {
                label: label.clone(),
                options: options.clone(),
                value: value.clone().unwrap_or_default(),
                key: key.clone().unwrap_or_default(),
            })
        }
        ElementType::Multiselect {
            label,
            options,
            values,
            key,
        } => {
            element::Type::Multiselect(MultiSelectElement {
                label: label.clone(),
                options: options.clone(),
                values: values.clone(),
                key: key.clone().unwrap_or_default(),
            })
        }
        ElementType::DateInput { label, value, key } => {
            element::Type::DateInput(DateInputElement {
                label: label.clone(),
                value: value.clone(),
                key: key.clone().unwrap_or_default(),
            })
        }
        ElementType::TimeInput { label, value, key } => {
            element::Type::TimeInput(TimeInputElement {
                label: label.clone(),
                value: value.clone(),
                key: key.clone().unwrap_or_default(),
            })
        }
        ElementType::ColorPicker { label, value, key } => {
            element::Type::ColorPicker(ColorPickerElement {
                label: label.clone(),
                value: value.clone(),
                key: key.clone().unwrap_or_default(),
            })
        }
        ElementType::FileUploader { label, key } => {
            element::Type::FileUploader(FileUploaderElement {
                label: label.clone(),
                key: key.clone().unwrap_or_default(),
            })
        }
        ElementType::CameraInput { label, key } => {
            element::Type::CameraInput(CameraInputElement {
                label: label.clone(),
                key: key.clone().unwrap_or_default(),
            })
        }
        ElementType::Json { value } => {
            element::Type::Json(JsonElement {
                data: serde_json::to_string(value).unwrap_or_default(),
            })
        }
        ElementType::Image { src, caption, width } => {
            element::Type::Image(ImageElement {
                src: src.clone(),
                caption: caption.clone().unwrap_or_default(),
                width: width.unwrap_or(0),
            })
        }
        ElementType::Audio { src } => {
            element::Type::Audio(AudioElement {
                src: src.clone(),
            })
        }
        ElementType::Video { src } => {
            element::Type::Video(VideoElement {
                src: src.clone(),
            })
        }
        ElementType::Container { children } => {
            element::Type::Container(ContainerElement {
                children: children.iter().map(|c| c.to_string()).collect(),
            })
        }
        ElementType::Column { children, width } => {
            element::Type::Column(ColumnElement {
                children: children.iter().map(|c| c.to_string()).collect(),
                width: width.unwrap_or(1.0),
            })
        }
        ElementType::Row { children } => {
            element::Type::Row(RowElement {
                children: children.iter().map(|c| c.to_string()).collect(),
            })
        }
        ElementType::Tab { label, children } => {
            element::Type::Tab(TabElement {
                label: label.clone(),
                children: children.iter().map(|c| c.to_string()).collect(),
            })
        }
        ElementType::Expander {
            label,
            expanded,
            children,
        } => {
            element::Type::Expander(ExpanderElement {
                label: label.clone(),
                expanded: *expanded,
                children: children.iter().map(|c| c.to_string()).collect(),
            })
        }
        ElementType::Tabs { tabs } => {
            element::Type::Tabs(TabsElement {
                tabs: tabs
                    .iter()
                    .map(|(label, children)| TabItem {
                        label: label.clone(),
                        children: children.iter().map(|c| c.to_string()).collect(),
                    })
                    .collect(),
            })
        }
        ElementType::Sidebar { children } => {
            element::Type::Sidebar(SidebarElement {
                children: children.iter().map(|c| c.to_string()).collect(),
            })
        }
        ElementType::Metric { label, value, delta } => {
            element::Type::Metric(MetricElement {
                label: label.clone(),
                value: value.clone(),
                delta: delta.clone().unwrap_or_default(),
            })
        }
        ElementType::Success { message } => {
            element::Type::Success(SuccessElement {
                message: message.clone(),
            })
        }
        ElementType::Error { message } => {
            element::Type::Error(ErrorElement {
                message: message.clone(),
            })
        }
        ElementType::Warning { message } => {
            element::Type::Warning(WarningElement {
                message: message.clone(),
            })
        }
        ElementType::Info { message } => {
            element::Type::Info(InfoElement {
                message: message.clone(),
            })
        }
        ElementType::Progress { value } => {
            element::Type::Progress(ProgressElement { value: *value })
        }
        ElementType::Dataframe { data } => {
            element::Type::Dataframe(DataFrameElement {
                data: data.clone(),
            })
        }
        ElementType::Table { headers, rows } => {
            element::Type::Table(TableElement {
                headers: headers.clone(),
                rows: rows
                    .iter()
                    .map(|row| TableRow {
                        cells: row.clone(),
                    })
                    .collect(),
            })
        }
        ElementType::Divider => {
            element::Type::Divider(DividerElement {})
        }
        ElementType::Empty => {
            element::Type::Empty(EmptyElement {})
        }
        ElementType::LineChart { data, title } => {
            element::Type::LineChart(LineChartElement {
                data: data.clone(),
                title: title.clone().unwrap_or_default(),
            })
        }
        ElementType::BarChart { data, title } => {
            element::Type::BarChart(BarChartElement {
                data: data.clone(),
                title: title.clone().unwrap_or_default(),
            })
        }
        ElementType::AreaChart { data, title } => {
            element::Type::AreaChart(AreaChartElement {
                data: data.clone(),
                title: title.clone().unwrap_or_default(),
            })
        }
        ElementType::ScatterChart { data, title } => {
            element::Type::ScatterChart(ScatterChartElement {
                data: data.clone(),
                title: title.clone().unwrap_or_default(),
            })
        }
        ElementType::PieChart { data, title } => {
            element::Type::PieChart(PieChartElement {
                data: data.clone(),
                title: title.clone().unwrap_or_default(),
            })
        }
        ElementType::PlotlyChart { spec } => {
            element::Type::PlotlyChart(PlotlyChartElement {
                spec: spec.clone(),
            })
        }
        ElementType::VegaLiteChart { spec } => {
            element::Type::VegaLiteChart(VegaLiteChartElement {
                spec: spec.clone(),
            })
        }
        ElementType::BokehChart { spec } => {
            element::Type::BokehChart(BokehChartElement {
                spec: spec.clone(),
            })
        }
    };

    Element {
        id: id_str,
        r#type: Some(element_type),
    }
}

/// Create a ForwardMsg with deltas
pub fn create_delta_msg(deltas: Vec<CoreDelta>) -> ForwardMsg {
    let delta_msgs = deltas
        .into_iter()
        .filter_map(|delta| match delta {
            CoreDelta::AddElement {
                id,
                element,
                parent_id,
            } => {
                let proto_element = element_type_to_proto(id.inner(), &element);
                Some(delta::Type::AddElement(AddElementDelta {
                    element: Some(proto_element),
                    parent_id: parent_id.map(|p| p.to_string()).unwrap_or_default(),
                }))
            }
            CoreDelta::UpdateElement { id, element } => {
                let proto_element = element_type_to_proto(id.inner(), &element);
                Some(delta::Type::UpdateElement(UpdateElementDelta {
                    element: Some(proto_element),
                }))
            }
            CoreDelta::RemoveElement { id } => {
                Some(delta::Type::RemoveElement(RemoveElementDelta {
                    element_id: id.to_string(),
                }))
            }
            CoreDelta::ClearContainer { id } => {
                Some(delta::Type::ClearContainer(ClearContainerDelta {
                    container_id: id.to_string(),
                }))
            }
        })
        .map(|delta_type| Delta {
            r#type: Some(delta_type),
        })
        .collect();

    ForwardMsg {
        hash: Uuid::new_v4().to_string(),
        r#type: Some(forward_msg::Type::Delta(DeltaMsg {
            deltas: delta_msgs,
        })),
    }
}

/// Convert deltas to JSON for frontend
pub fn deltas_to_json(deltas: Vec<CoreDelta>) -> serde_json::Value {
    let elements: Vec<serde_json::Value> = deltas
        .into_iter()
        .filter_map(|delta| match delta {
            CoreDelta::AddElement {
                id,
                element,
                parent_id,
            } => {
                Some(serde_json::json!({
                    "type": "add_element",
                    "id": id.inner().to_string(),
                    "parent_id": parent_id.map(|p| p.to_string()),
                    "element": element_to_json(&element),
                }))
            }
            CoreDelta::UpdateElement { id, element } => {
                Some(serde_json::json!({
                    "type": "update_element",
                    "id": id.inner().to_string(),
                    "element": element_to_json(&element),
                }))
            }
            CoreDelta::RemoveElement { id } => {
                Some(serde_json::json!({
                    "type": "remove_element",
                    "id": id.inner().to_string(),
                }))
            }
            CoreDelta::ClearContainer { id } => {
                Some(serde_json::json!({
                    "type": "clear_container",
                    "id": id.inner().to_string(),
                }))
            }
        })
        .collect();

    serde_json::json!({
        "type": "delta",
        "elements": elements,
    })
}

/// Convert ElementType to JSON
fn element_to_json(element: &ElementType) -> serde_json::Value {
    match element {
        ElementType::Text { value } => {
            serde_json::json!({
                "type": "text",
                "value": value,
            })
        }
        ElementType::Markdown { value } => {
            serde_json::json!({
                "type": "markdown",
                "value": value,
            })
        }
        ElementType::Code { value, language } => {
            serde_json::json!({
                "type": "code",
                "value": value,
                "language": language,
            })
        }
        ElementType::Heading { value, level } => {
            serde_json::json!({
                "type": "heading",
                "value": value,
                "level": level,
            })
        }
        ElementType::Button { label, key } => {
            serde_json::json!({
                "type": "button",
                "label": label,
                "key": key,
            })
        }
        ElementType::TextInput { label, value, key } => {
            serde_json::json!({
                "type": "text_input",
                "label": label,
                "value": value,
                "key": key,
            })
        }
        ElementType::TextArea { label, value, key } => {
            serde_json::json!({
                "type": "text_area",
                "label": label,
                "value": value,
                "key": key,
            })
        }
        ElementType::NumberInput { label, value, key } => {
            serde_json::json!({
                "type": "number_input",
                "label": label,
                "value": value,
                "key": key,
            })
        }
        ElementType::Slider { label, value, min, max, key } => {
            serde_json::json!({
                "type": "slider",
                "label": label,
                "value": value,
                "min": min,
                "max": max,
                "key": key,
            })
        }
        ElementType::Checkbox { label, value, key } => {
            serde_json::json!({
                "type": "checkbox",
                "label": label,
                "value": value,
                "key": key,
            })
        }
        ElementType::Selectbox { label, options, value, key } => {
            serde_json::json!({
                "type": "selectbox",
                "label": label,
                "options": options,
                "value": value,
                "key": key,
            })
        }
        ElementType::Multiselect { label, options, values, key } => {
            serde_json::json!({
                "type": "multiselect",
                "label": label,
                "options": options,
                "values": values,
                "key": key,
            })
        }
        ElementType::Json { value } => {
            serde_json::json!({
                "type": "json",
                "value": value,
            })
        }
        ElementType::Image { src, caption, width } => {
            serde_json::json!({
                "type": "image",
                "src": src,
                "caption": caption,
                "width": width,
            })
        }
        ElementType::Divider => {
            serde_json::json!({
                "type": "divider",
            })
        }
        ElementType::Empty => {
            serde_json::json!({
                "type": "empty",
            })
        }
        ElementType::Success { message } => {
            serde_json::json!({
                "type": "success",
                "message": message,
            })
        }
        ElementType::Error { message } => {
            serde_json::json!({
                "type": "error",
                "message": message,
            })
        }
        ElementType::Warning { message } => {
            serde_json::json!({
                "type": "warning",
                "message": message,
            })
        }
        ElementType::Info { message } => {
            serde_json::json!({
                "type": "info",
                "message": message,
            })
        }
        ElementType::Metric { label, value, delta } => {
            serde_json::json!({
                "type": "metric",
                "label": label,
                "value": value,
                "delta": delta,
            })
        }
        ElementType::Progress { value } => {
            serde_json::json!({
                "type": "progress",
                "value": value,
            })
        }
        ElementType::Column { .. } => {
            serde_json::json!({
                "type": "column",
            })
        }
        ElementType::Row { .. } => {
            serde_json::json!({
                "type": "row",
            })
        }
        ElementType::Tab { .. } => {
            serde_json::json!({
                "type": "tab",
            })
        }
        ElementType::Expander { .. } => {
            serde_json::json!({
                "type": "expander",
            })
        }
        ElementType::Sidebar { .. } => {
            serde_json::json!({
                "type": "sidebar",
            })
        }
        ElementType::Container { .. } => {
            serde_json::json!({
                "type": "container",
            })
        }
        ElementType::Radio { label, options, value, key } => {
            serde_json::json!({
                "type": "radio",
                "label": label,
                "options": options,
                "value": value,
                "key": key,
            })
        }
        ElementType::DateInput { label, value, key } => {
            serde_json::json!({
                "type": "date_input",
                "label": label,
                "value": value,
                "key": key,
            })
        }
        ElementType::TimeInput { label, value, key } => {
            serde_json::json!({
                "type": "time_input",
                "label": label,
                "value": value,
                "key": key,
            })
        }
        ElementType::ColorPicker { label, value, key } => {
            serde_json::json!({
                "type": "color_picker",
                "label": label,
                "value": value,
                "key": key,
            })
        }
        ElementType::FileUploader { label, key } => {
            serde_json::json!({
                "type": "file_uploader",
                "label": label,
                "key": key,
            })
        }
        ElementType::Dataframe { data } => {
            serde_json::json!({
                "type": "dataframe",
                "data": data,
            })
        }
        ElementType::Table { headers, rows } => {
            serde_json::json!({
                "type": "table",
                "headers": headers,
                "rows": rows,
            })
        }
        ElementType::CameraInput { label, key } => {
            serde_json::json!({
                "type": "camera_input",
                "label": label,
                "key": key,
            })
        }
        ElementType::Audio { src } => {
            serde_json::json!({
                "type": "audio",
                "src": src,
            })
        }
        ElementType::Video { src } => {
            serde_json::json!({
                "type": "video",
                "src": src,
            })
        }
        ElementType::Tabs { tabs } => {
            serde_json::json!({
                "type": "tabs",
                "tabs": tabs,
            })
        }
        ElementType::LineChart { .. } => {
            serde_json::json!({
                "type": "line_chart",
            })
        }
        ElementType::BarChart { .. } => {
            serde_json::json!({
                "type": "bar_chart",
            })
        }
        ElementType::AreaChart { .. } => {
            serde_json::json!({
                "type": "area_chart",
            })
        }
        ElementType::ScatterChart { .. } => {
            serde_json::json!({
                "type": "scatter_chart",
            })
        }
        ElementType::PieChart { .. } => {
            serde_json::json!({
                "type": "pie_chart",
            })
        }
        ElementType::PlotlyChart { .. } => {
            serde_json::json!({
                "type": "plotly_chart",
            })
        }
        ElementType::VegaLiteChart { .. } => {
            serde_json::json!({
                "type": "vega_lite_chart",
            })
        }
        ElementType::BokehChart { .. } => {
            serde_json::json!({
                "type": "bokeh_chart",
            })
        }
    }
}

/// Create a NewSessionMsg
pub fn create_session_msg(session_id: &str, script_hash: &str) -> ForwardMsg {
    ForwardMsg {
        hash: Uuid::new_v4().to_string(),
        r#type: Some(forward_msg::Type::NewSession(NewSessionMsg {
            session_id: session_id.to_string(),
            script_hash: script_hash.to_string(),
            elements: vec![],
        })),
    }
}

/// Serialize ForwardMsg to bytes
pub fn serialize_forward_msg(msg: &ForwardMsg) -> Result<Vec<u8>, prost::EncodeError> {
    let mut buf = Vec::new();
    msg.encode(&mut buf)?;
    Ok(buf)
}

/// Deserialize BackMsg from bytes
pub fn deserialize_back_msg(bytes: &[u8]) -> Result<BackMsg, prost::DecodeError> {
    BackMsg::decode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use platypus_core::element::ElementId;

    #[test]
    fn test_element_type_to_proto_text() {
        let element = ElementType::Text {
            value: "Hello".to_string(),
        };
        let proto = element_type_to_proto(1, &element);
        assert_eq!(proto.id, "1");
    }

    #[test]
    fn test_create_delta_msg() {
        let delta = CoreDelta::AddElement {
            id: ElementId::new(1),
            element: ElementType::Text {
                value: "Test".to_string(),
            },
            parent_id: None,
        };
        let msg = create_delta_msg(vec![delta]);
        assert!(!msg.hash.is_empty());
    }

    #[test]
    fn test_serialize_forward_msg() {
        let msg = create_session_msg("session123", "hash456");
        let bytes = serialize_forward_msg(&msg).unwrap();
        assert!(!bytes.is_empty());
    }
}
