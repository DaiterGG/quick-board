## Quick Board
A fast, lightweight, single executable program for sketching and drawing, written in Rust.

![qb](https://github.com/user-attachments/assets/e7f89053-c8b6-4cc4-af34-4124c2246761)

## GUI implementation
Compile time*, retained mode ui layer.

<img align="right" width="157" height="398" alt="Esc" src="https://github.com/user-attachments/assets/c7d9358f-b9bd-4ee9-ac9f-fc6f22830360" />

``` rust
markup! {{
    RootMain:Div {
        Header:Div,
        RightWide:Div {
            ColorPicker:Div {
                HSV_H:Slider {
                  HSV_H_Handle:Div
                },
                HSV_S:Slider {
                  HSV_S_Handle:Div
                },
                HSV_V:Slider {
                  HSV_V_Handle:Div
                }
            },
        }
    } 
}}
```
``` rust
HSV_H: Align [A::block(Vertical, Start, V::new(Percent, 34))],
HSV_H: Display [D::idle(Data::transparent().with_tex(RangeHue))],
HSV_H: Slider [Slider::new(HSV_H_Handle.into()).within()],
HSV_H: Subscribe [Callback::HSVHue],
```
## Infinite draw canvas
Virtualization of the textures

<img align="right" width="549" height="503" alt="draw" src="https://github.com/user-attachments/assets/becf454e-9a02-4bba-acd3-8caa0b511d09" />

``` rust
pub struct HistoryStep {
    rows: Vec<TextureRow>,
    rows_offset: i32,

    // copy of units to draw them in a loop
    flat_copy: Vec<TextureUnit>,
}
struct TextureRow {
    units: Vec<Option<TextureUnit>>,
    row_offset: i32,
}

```
Editable history and layers support
``` rust
pub struct History {
    pub steps: Vec<HistoryStep>,
    pub selected_h_step: Option<usize>,
    pub layers: Vec<Layer>,
    pub selected_layer: Option<usize>,
}
```
## Other Technical details:
* This app uses the [rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2) for its GUI and drawing functionality.
