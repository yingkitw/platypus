//! Performance benchmarks for element operations.
//!
//! Run with: cargo bench --bench element_benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use platypus_core::element::ElementId;
use platypus_core::elements::{
    TextElement, ButtonElement, TextInputElement, SliderElement, CheckboxElement,
    ContainerElement, ResponsiveContainerElement, ThemedButtonElement,
};
use platypus_core::traits::{Renderable, Validatable, Interactive, InteractionEvent, Container, Responsive, Themeable};

fn benchmark_element_creation(c: &mut Criterion) {
    c.bench_function("create_text_element", |b| {
        b.iter(|| {
            let id = black_box(ElementId::new(1));
            TextElement::new(id, "Hello, World!")
        })
    });

    c.bench_function("create_button_element", |b| {
        b.iter(|| {
            let id = black_box(ElementId::new(1));
            ButtonElement::new(id, "Click me")
        })
    });

    c.bench_function("create_slider_element", |b| {
        b.iter(|| {
            let id = black_box(ElementId::new(1));
            SliderElement::new(id, "Volume", 0.0, 100.0)
        })
    });
}

fn benchmark_element_serialization(c: &mut Criterion) {
    c.bench_function("text_element_to_json", |b| {
        let element = TextElement::new(ElementId::new(1), "Hello");
        b.iter(|| element.to_json())
    });

    c.bench_function("button_element_to_json", |b| {
        let element = ButtonElement::new(ElementId::new(1), "Click");
        b.iter(|| element.to_json())
    });

    c.bench_function("slider_element_to_json", |b| {
        let element = SliderElement::new(ElementId::new(1), "Volume", 0.0, 100.0).unwrap();
        b.iter(|| element.to_json())
    });
}

fn benchmark_element_validation(c: &mut Criterion) {
    c.bench_function("text_element_validate", |b| {
        let element = TextElement::new(ElementId::new(1), "Hello");
        b.iter(|| element.validate())
    });

    c.bench_function("button_element_validate", |b| {
        let element = ButtonElement::new(ElementId::new(1), "Click");
        b.iter(|| element.validate())
    });

    c.bench_function("slider_element_validate", |b| {
        let element = SliderElement::new(ElementId::new(1), "Volume", 0.0, 100.0).unwrap();
        b.iter(|| element.validate())
    });
}

fn benchmark_interactive_operations(c: &mut Criterion) {
    c.bench_function("button_handle_click_event", |b| {
        let mut button = ButtonElement::new(ElementId::new(1), "Click");
        let event = InteractionEvent {
            event_type: "click".to_string(),
            target_id: ElementId::new(1),
            data: None,
            timestamp: 0,
        };
        b.iter(|| button.handle_event(&event))
    });

    c.bench_function("text_input_set_value", |b| {
        let mut input = TextInputElement::new(ElementId::new(1), "Name");
        let value = serde_json::Value::String("John Doe".to_string());
        b.iter(|| input.set_value(value.clone()))
    });

    c.bench_function("slider_set_value", |b| {
        let mut slider = SliderElement::new(ElementId::new(1), "Volume", 0.0, 100.0).unwrap();
        let value = serde_json::json!(50.0);
        b.iter(|| slider.set_value(value.clone()))
    });

    c.bench_function("checkbox_handle_change_event", |b| {
        let mut checkbox = CheckboxElement::new(ElementId::new(1), "Accept");
        let event = InteractionEvent {
            event_type: "change".to_string(),
            target_id: ElementId::new(1),
            data: Some(serde_json::Value::Bool(true)),
            timestamp: 0,
        };
        b.iter(|| checkbox.handle_event(&event))
    });
}

fn benchmark_container_operations(c: &mut Criterion) {
    c.bench_function("container_add_child", |b| {
        let mut container = ContainerElement::new(ElementId::new(1));
        b.iter(|| {
            let id = black_box(ElementId::new(2));
            container.add_child(id)
        })
    });

    c.bench_function("container_with_10_children", |b| {
        b.iter(|| {
            let mut container = ContainerElement::new(ElementId::new(1));
            for i in 2..12 {
                let _ = container.add_child(ElementId::new(i as u64));
            }
            container
        })
    });
}

fn benchmark_responsive_operations(c: &mut Criterion) {
    c.bench_function("responsive_container_creation", |b| {
        b.iter(|| ResponsiveContainerElement::new(ElementId::new(1)))
    });

    c.bench_function("responsive_get_breakpoints", |b| {
        let container = ResponsiveContainerElement::new(ElementId::new(1));
        b.iter(|| container.breakpoints())
    });

    c.bench_function("responsive_layout_for_breakpoint", |b| {
        let container = ResponsiveContainerElement::new(ElementId::new(1));
        b.iter(|| container.layout_for_breakpoint("mobile"))
    });
}

fn benchmark_themeable_operations(c: &mut Criterion) {
    c.bench_function("themed_button_creation", |b| {
        b.iter(|| ThemedButtonElement::new(ElementId::new(1), "Click"))
    });

    c.bench_function("themed_button_set_theme", |b| {
        let mut button = ThemedButtonElement::new(ElementId::new(1), "Click");
        b.iter(|| button.set_theme("dark"))
    });

    c.bench_function("themed_button_to_json", |b| {
        let button = ThemedButtonElement::new(ElementId::new(1), "Click");
        b.iter(|| button.to_json())
    });
}

fn benchmark_batch_operations(c: &mut Criterion) {
    c.bench_function("create_100_text_elements", |b| {
        b.iter(|| {
            (0..100)
                .map(|i| TextElement::new(ElementId::new(i), "Hello"))
                .collect::<Vec<_>>()
        })
    });

    c.bench_function("serialize_100_elements", |b| {
        let elements: Vec<_> = (0..100)
            .map(|i| TextElement::new(ElementId::new(i), "Hello"))
            .collect();
        b.iter(|| {
            elements.iter().map(|e| e.to_json()).collect::<Vec<_>>()
        })
    });

    c.bench_function("validate_100_elements", |b| {
        let elements: Vec<_> = (0..100)
            .map(|i| TextElement::new(ElementId::new(i), "Hello"))
            .collect();
        b.iter(|| {
            elements.iter().map(|e| e.validate()).collect::<Vec<_>>()
        })
    });
}

criterion_group!(
    benches,
    benchmark_element_creation,
    benchmark_element_serialization,
    benchmark_element_validation,
    benchmark_interactive_operations,
    benchmark_container_operations,
    benchmark_responsive_operations,
    benchmark_themeable_operations,
    benchmark_batch_operations,
);

criterion_main!(benches);
