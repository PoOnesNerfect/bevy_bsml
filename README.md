# bevy_bsml

[<img alt="github" src="https://img.shields.io/badge/github-poonesnerfect/bevy_bsml-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/poonesnerfect/bevy_bsml)
[<img alt="crates.io" src="https://img.shields.io/crates/v/bevy_bsml.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/bevy_bsml)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bevy_bsml-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/bevy_bsml)

**BSML** stands for **B**evy's **S**imple **M**arkup **L**anguage, or just **BS** **M**arkup **L**anguage.

**bevy_bsml** allows you to compose UI elements using simple markup language, inspired by svelte and tailwindcss.

It's built on top of the official [bevy_ui](https://github.com/bevyengine/bevy?tab=readme-ov-file) library, so you can still use `bevy_ui` to manually interact with the ui node or styles.

To see the basic usages of bevy_bsml, check out examples:

- [basic menu](https://github.com/poonesnerfect/bevy_bsml/blob/main/examples/basic_menu.rs)
- [loading bar](https://github.com/poonesnerfect/bevy_bsml/blob/main/examples/loading_bar.rs)

**For Breaking Changes and New Features, see [CHANGELOG.md](https://github.com/PoOnesNerfect/bevy_bsml/blob/main/CHANGELOG.md)**

## Table of Contents

<!--toc:start-->

- [bevy_bsml](#bevybsml)
  - [Table of Contents](#table-of-contents)
  - [Why not HTML or XML?](#why-not-html-or-xml)
  - [Supported Bevy Versions](#supported-bevy-versions)
  - [Setup](#setup)
  - [Basics of BSML](#basics-of-bsml)
  - [Element Types in BSML](#element-types-in-bsml)
    - [node](#node)
    - [for](#for)
    - [slot](#slot)
    - [text](#text)
  - [Attributes in BSML](#attributes-in-bsml)
    - [class](#class)
      - [Changing Styles on Interaction](#changing-styles-on-interaction)
    - [labels](#labels)
  - [Custom Reusable Components](#custom-reusable-components)
    - [Defining a Reusable Component](#defining-a-reusable-component)
    - [Using Component Fields in BSML](#using-component-fields-in-bsml)
    - [Using a Reusable Component](#using-a-reusable-component)
      - [Spawning component directly](#spawning-component-directly)
      - [Including in other bsml elements](#including-in-other-bsml-elements)
  - [Spawning UI Elements using BSML](#spawning-ui-elements-using-bsml)
    - [Spawning an anonymous UI element](#spawning-an-anonymous-ui-element)
    - [Spawning a Reusable Component](#spawning-a-reusable-component)
  - [Despawning BSML Elements](#despawning-bsml-elements)
  - [Reactivity with Spawned Components](#reactivity-with-spawned-components)

<!--toc:end-->

## Why not HTML or XML?

Because the angle bracketed markup languages don't work well with rust macros, and `(...)` and `{...}` work naturally.

## Supported Bevy Versions

From version 0.14, bevy_bsml will keep the same minor version as Bevy,
while patch version will vary.

| Bevy | bevy_bsml |
| ---- | --------- |
| 0.14 | 0.14      |
| 0.13 | 0.0.8     |
| 0.12 | 0.0.7     |

## Setup

Import the prelude module, and add `BsmlPlugin`.

```rust
use bevy_bsml::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(BsmlPlugin)
        .run();
}
```

See the [examples](https://github.com/PoOnesNerfect/bevy_bsml/tree/main/examples) for more detailed usage.

## Basics of BSML

In BSML, `(...)` contains the name of the element and its attributes, and `{...}` contains the content of the element, like nested elements.

Here is a basic example of BSML:

```
(node) {
    (text) { "hello world" }
}
```

which is equal to following html:

```html
<div>
  hello world
</div>
```

If an element doesn't have any content, you can skip the braces:

```
(node)
```

This is a valid bsml; it will spawn a NodeBundle with default styles with no content.

You can also nest elements inside braces:

```
(node) {
    (node)
    (text) { "hello world" }
    (node) {
        (text) { "nested text" }
    }
}
```

is equal to

```html
<div>
  <div />
  hello world
  <div>
    nested text
  </div>
</div>
```

## Element Types in BSML

### node

`(node)` is like `div` in html; you can have nested elements in it, or just use one without nested elements just to style it.

Available attributes are [labels](#labels) and [class](#class).

**Examples**:

_100x100px blue box with no nested elements_:

```
(node class=[w(100.0), h(100.0), BG_BLUE_400])
```

_centering a node_:

```
(node class=[W_FULL, H_FULL, JUSTIFY_CENTER, ITEMS_CENTER]) {
    (node class=[h(100.0), w(100.0), BG_BLUE_400])
}
```

_text in the blue box_:

```
(node class=[w(100.0), h(100.0), BG_BLUE_400]) {
    (text class=[TEXT_WHITE]}) { "hello world" }
}
```

_with labels_:

```rust
#[derive(Component)]
pub struct MyNode { i: u8 }
```

```
(node labels=[MyNode { i: 0 }] class=[w(100.0), h(100.0), BG_BLUE_400])
```

### for

`(for)` is a node that iterates over a given iterator and repeats nested elements for each item.

The syntax is `(for {<item> in <iterator>}) { <nested elements> }`.

`<item>` is a variable name, and
`<iterator>` can be any expression that implements `IntoIterator`, just like in `for ... in ...` loop.

Optionally, you can also get the index of the item like:

```
(for {<index>, <item> in <iterator>}) { <nested elements> }
```

Available attributes are [labels](#labels) and [class](#class).

**Examples**:

_simple menu screen_:

```
(for
    {name in ["Continue", "Setting", "Exit"]}
    class=[W_FULL, H_FULL, JUSTIFY_CENTER, ITEMS_CENTER, BG_WHITE]
) {
    (node class=[w(100.0), h(100.0), BG_BLUE_400]) {
        (text class=[TEXT_BASE]) { "{}", name }
    }
}
```

_simple menu screen with index_:

```
(for
    {i, name in ["Continue", "Setting", "Exit"]}
    class=[W_FULL, H_FULL, JUSTIFY_CENTER, ITEMS_CENTER, BG_WHITE]
) {
    (node class=[w(100.0), h(100.0), BG_BLUE_400]) {
        (text class=[TEXT_BASE]) { "{}: {}", i, name }
    }
}
```

### slot

`(slot)` is a special element; it's used in [reusable component](#custom-reusable-components) definitions to expose space for its child elements.

When it has nested elements in braces, it will be used as default content of the slot, replaced when the slot is used.
If you don't need default content, you can skip the braces.

Available attributes are [labels](#labels) and [class](#class).

**Examples**:

_Define a Button component_:

```rust
#[derive(Component)]
pub struct MyButton;

// define a button component
bsml!{MyButton;
    (slot class=[w(100.0), h(100.0), BG_BLUE_400, hovered(BG_BLUE_600)]) {
        (text class=[TEXT_WHITE]) { "I am button" } // default content
    }
}

fn spawn_ui_system(mut commands: Commands) {
    // spawn a screen using bsml!
    commands.spawn_bsml(bsml!(
        (node class=[W_FULL, H_FULL, JUSTIFY_CENTER, ITEMS_CENTER, BG_TRANSPARENT]) {
            (MyButton) { (text class=[TEXT_WHITE]) { "button 1" } }    // displays "button 1"
            (MyButton) { (text class=[TEXT_WHITE]) { "button 2" } }    // displays "button 2"
            (MyButton)                              // displays "i am button"
        }
    ));
}
```

### text

`(text)` element is different than other elements in that you cannot nest elements inside it.

Instead, content inside `{...}` is exactly like arguments of `format!(...)`.

Available attributes are [labels](#labels) and [class](#class).

**Examples**:

_basic_:

```
(text) { "hello world" }
```

_with arguments_:

```
(text) { "{} + {} = {}", 1, 2, 1 + 2 }
```

_with styling_:

```
(text class=[TEXT_XS]) { "I'm a tiny wittle text" }
```

_with labels_:

```rust
#[derive(Component)]
pub struct MyText;
```

```
(text labels=[MyText] class=[TEXT_XS]) { "I'm a tiny little text" }
```

## Attributes in BSML

### class

`class` attribute is used to specify the styles of the element, like tailwindcss.

In class attribute, you can provide list of provided style classes like [W_FULL] or [BG_SLATE_200],
or any expressions that return of one: [StyleClass], [BackgroundColorClass], [BorderColorClass], and [ZIndex].

**Example** Centering a node in the center of the screen

```
(node class=[W_FULL, H_FULL, JUSTIFY_CENTER, ITEMS_CENTER, BG_TRANSPARENT]) {
    (node class=[h(100.0), w(100.0), BG_BLUE_400])
}
```

The outer node will fill the entire screen, and center its content.
The inner node will be 100x100px with blue background color.

#### Changing Styles on Interaction

You can specify styles that are applied when the node is hovered or pressed, like in tailwindcss.

```
(node class=[h(100.0), w(100.0), BG_BLUE_400, hovered(BG_BLUE_600), pressed(BG_BLUE_800)])
```

**caveat**: if you plan to use `hovered` or `pressed` style classes, you must specify also specify the base class,
else it will not return back to previous style.

### labels

`labels` attribute is a list of bevy components that are spawned with the node.

You can use these components to query for the node, or change data when the node is hovered or pressed.

You can then query for the node using the label component.

```
(node labels=[MyComponent])
```

```rust
#[derive(Component)]
pub struct MyComponent;

// get the entity of the node with MyComponent
// when the node is clicked or hovered
fn my_system(query: Query<Entity, (With<MyComponent>, Changed<Interaction>)>) {
    // ...
}
```

If your label component has fields, you can also provide expression that returns the initialized component.

```
(node labels=[MyComponent { i: 0, name: "hello".to_owned() }, label2()])
```

```rust
#[derive(Component)]
pub struct MyComponent {
    pub i: u8,
    pub name: String,
}

#[derive(Component)]
pub struct Label2;

fn label2() -> Label2 {
    Label2
}
```

and you can interact with the components in your systems:

```rust
fn my_system(query: Query<&MyComponent, (With<Label2>, Changed<Interaction>)>) {
    for my_component in query.iter() {
        println!("Interacted with node {}: {}", my_component.i, my_component.name);
    }
}
```

**advanced**: you can also use the fields of the label component in your bsml:

```rust
#[derive(Component)]
pub struct Label {
    pub text: &'static str,
    pub width: f32
}
```

```
(node labels=[Label { text: "hello world", width: 100.0 }]) {
    (text class=[w_px(labels.0.width)]) { "{}", labels.0.text }
}
```

## Custom Reusable Components

### Defining a Reusable Component

You can define your own reusable components using bsml.

The only requirement is that you must derive `bevy::prelude::Component` trait on the struct.

Here is an example of a reusable component definition:

```rust
#[derive(Component)]
pub struct MyComponent {
    pub i: u8,
    pub name: &'static str,
}

bsml! {MyComponent;
    (node class=[h_px(100.0), w_px(100.0), BG_BLUE_400]) {
        (text class=[TEXT_WHITE, TEXT_BASE]) { "index: {}, name: {}", self.i, self.name }
    }
}
```

In the macro, notice the component struct followed by semicolon, which is not part of the bsml syntax.

This may seem a bit jarring, but this is intentional to make it clear that you are defining a reusable component.

### Using Component Fields in BSML

As you may have noticed, you can also use the fields of the component in bsml.

You can use it in class, labels, or in nested element contents like text element, by referencing them like `self.<field_name>`.

```rust
#[derive(Component)]
pub struct MyComponent {
    pub i: u8,
    pub name: &'static str,
    pub width: f32,
}

bsml! {MyComponent;
    (node class=[h_px(100.0), w_px(self.width), BG_BLUE_400]) {
        (text class=[TEXT_WHITE, TEXT_BASE]) { "index: {}, name: {}", self.i, self.name }
    }
}
```

**note** that these are not reactive. They are evaluated only once when the component is initialized.

Even if the field value changes, the bsml UI will not be updated.

### Using a Reusable Component

After you have defined a reusable component, you can use it either by spawning it directly or including in other bsml elements.

#### Spawning component directly

The easiest way to use a reusable component is to spawn it directly.

```rust
use bevy_bsml::prelude::*;

fn spawn_bsml_ui(mut commands: Commands) {
    commands.spawn_bsml(MyComponent { i: 0, name: "hello" });
}
```

#### Including in other bsml elements

When using a reusable component in other bsml elements, you can use any expression that returns the initialized component in a parenthesis.

```rust
use bevy_bsml::prelude::*;

#[derive(Component)]
pub struct MyContainer;

bsml! {MyContainer;
    (node class=[W_FULL, H_FULL, JUSTIFY_CENTER, ITEMS_CENTER, BG_TRANSPARENT]) {
        (MyComponent { i: 0, name: "hello" })
    }
}

fn spawn_bsml_ui(mut commands: Commands) {
    commands.spawn_bsml(MyContainer);
}
```

The expression can be anything. You could even do something like:

```rust
use bevy_bsml::prelude::*;

#[derive(Component)]
pub struct MyContainer;

fn component(i: u8, name: &'static str) -> MyComponent {
    MyComponent { i, name }
}

bsml! {MyContainer;
    (node class=[W_FULL, H_FULL, JUSTIFY_CENTER, ITEMS_CENTER, BG_TRANSPARENT]) {
        (component(0, "hello"))
    }
}

fn spawn_bsml_ui(mut commands: Commands) {
    commands.spawn_bsml(MyContainer);
}
```

## Spawning UI Elements using BSML

There are two ways to spawn UI elements using bsml.

### Spawning an anonymous UI element

You can directly include the `bsml!` macro in `Commands::spawn_bsml` method.

```rust
use bevy_bsml::prelude::*;

fn spawn_bsml_ui(mut commands: Commands) {
    commands.spawn_bsml(bsml!(
        (node class=[W_FULL, H_FULL, JUSTIFY_CENTER, ITEMS_CENTER, BG_TRANSPARENT]) {
            (node class=[h_px(100.0), w_px(100.0), BG_BLUE_400])
        }
    ));
}
```

### Spawning a Reusable Component

See [Spawning Component Directly](#spawning-component-directly).

## Despawning BSML Elements

To despawn bsml elements, you can use `Commands::despawn_bsml` method, and provide the component's entity.

```rust
use bevy_bsml::prelude::*;

fn spawn_bsml_ui(mut commands: Commands) {
    // spawn a screen using bsml!
    let entity = commands.spawn_bsml(bsml!(
        (node class=[W_FULL, H_FULL, JUSTIFY_CENTER, ITEMS_CENTER, BG_TRANSPARENT]) {
            (node class=[h_px(100.0), w_px(100.0), BG_BLUE_400])
        }
    ))
    .id();

    // despawn the screen entity
    commands.despawn_bsml(entity);
}
```

## Reactivity with Spawned Components

Since bsml spawns bev_ui components internally, you can just use `bevy::ui::Interaction` to detect and react to UI interactions.

Check out examples to see how to react to UI interactions:

- [basic menu](https://github.com/poonesnerfect/bevy_bsml/blob/main/examples/basic_menu.rs)
- [loading bar](https://github.com/poonesnerfect/bevy_bsml/blob/main/examples/loading_bar.rs)

[BG_SLATE_200]: https://docs.rs/bevy_bsml/0.0.5/bevy_bsml/class/background_color/constant.BG_SLATE_200.html
[W_FULL]: https://docs.rs/bevy_bsml/0.0.5/bevy_bsml/class/sizing/constant.W_FULL.html
[StyleClass]: https://docs.rs/bevy_bsml/0.0.5/bevy_bsml/class/enum.StyleClass.html
[BackgroundColorClass]: https://docs.rs/bevy_bsml/0.0.5/bevy_bsml/class/background_color/struct.BackgroundColorClass.html
[BorderColorClass]: https://docs.rs/bevy_bsml/0.0.5/bevy_bsml/class/border_color/struct.BorderColorClass.html
[ZIndex]: https://docs.rs/bevy/0.12.1/bevy/ui/enum.ZIndex.html
