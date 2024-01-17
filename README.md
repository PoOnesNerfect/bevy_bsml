# bevy_bsml

[<img alt="github" src="https://img.shields.io/badge/github-poonesnerfect/bevy_bsml-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/poonesnerfect/bevy_bsml)
[<img alt="crates.io" src="https://img.shields.io/crates/v/bevy_bsml.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/bevy_bsml)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bevy_bsml-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/bevy_bsml)

**bevy_bsml** is a UI library built on top of bevy's official [bevy_ui](https://github.com/bevyengine/bevy?tab=readme-ov-file) library to help easily create and reuse styled UI components.

bevy_bsml is very much inspired by svelte and tailwindcss. So, if you're a web dev, you may feel right at home, with components and inline styling.

## Basic Usage

```rust
#[derive(Debug, Clone, Component)]
pub struct Menu {
    pub items: &'static [&'static str],
}

bsml! {Menu;
    (node class=[W_FULL, H_FULL, JUSTIFY_CENTER, ITEMS_CENTER, BG_TRANSPARENT]) {
        (for {i, name in self.items} class=[FLEX_COL, gap_y(20.0)]) {
            (MenuItem i name ..default)
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct MenuItem {
    pub i: usize,
    pub name: &'static str,
    pub width: f32,
}

impl Default for MenuItem {
    fn default() -> Self {
        Self {
            i: 0,
            name: "Menu Item",
            width: 200.0,
        }
    }
}

bsml! {MenuItem;
    (slot
        class=[w_px(self.width), h_px(75.0), BG_BLUE_400, hovered(BG_BLUE_600), pressed(BG_BLUE_800), JUSTIFY_CENTER, ITEMS_CENTER]
    ) {
        (text class=[TEXT_BASE]) { "{}", self.name }
    }
}

fn menu_item_system(
    query: Query<(&Interaction, &MenuItem), Changed<Interaction>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, item) in query.iter() {
        if *interaction == Interaction::Pressed {
            println!("Pressed: {}", item.name);

            if item.name == "Exit" {
                println!("exiting game...");
                exit.send(AppExit);
            }
        }
    }
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn_bsml(Menu {
        items: &["Continue", "Setting", "Exit"],
    });
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BsmlPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_ui)
        .add_systems(Update, menu_item_system)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
```

### What is BSML?

**BSML** stands for **B**evy's **S**imple **M**arkup **L**anguage, or **BS** **M**arkup **L**anguage; feel free to pick the one you like more.

#### Why not HTML or XML?

Because the angle bracketed markup languages don't work well with rust macros, and `(...)` and `{...}` work naturally.

### BSML Overview

#### 1. Tag is inside parenthesis `(MyComponent)`, followed by content nested in braces `{ ... }`.

```rust
bsml!{
    (node) {
        (node)
    }
}
```
is same as
```html
<div>
  <div />
</div>
```

You can have multiple elements inside braces:
```rust
bsml!{
    (node) {
        (node)
        (text) { "hello world" }
        (node) {
            (text) { "nested text" }
        }
    }
}
```
is same as
```html
<div>
  <div />
  hello world
  <div>
    nested text
  </div>
</div>
```

#### 2. There can be only one root element.

Valid cases:
```rust
bsml!{
    (node) { (node) }
}

bsml!{
  (node)
}

bsml!{
  (slot)
}

bsml!{
  (text) { "hello world" }
}
```

Invalid:
```rust
bsml!{
    (node) (node)
}

bsml!{
    (node) { (node) }
    (text) { "hello world" }
}
```

As you can see, if element doesn't have any nested elements, braces are optional.

So, if you have a UI Component `MenuItem`, which already has content layed out, you can skip braces when using the component.

```rust
pub struct MenuItem;
bsml!(MenuItem;
    (node) {
        (text) { "hello world" }
    }
)

pub struct Menu;
bsml!{Menu;
    (node) {
        (MenuItem)
        (MenuItem)
        (MenuItem)
    }
}
```

#### 3. Building Blocks

Currently, bsml has 3 base elements: **node**, **text**, and **slot**.

**node** is like `div` in html; you can have nested elements in it, or just use one without nested elements just to style it.

```rust
bsml! {
    (node class=[W_FULL, H_FULL, JUSTIFY_CENTER, ITEMS_CENTER, BG_WHITE]) {
        (node class=[h_px(100.0), w_px(100.0), BG_BLUE_400]) // I just centered this div, i mean node
    }
}
```

**text** is an element for displaying text. This element is different than others.
You must follow up the tag with braces, where its content is a string with optional arguments, exactly like arguments of `format!(...)`.
```rust
#[derive(Component)]
pub struct MyComponent;
bsml! {MyComponent;
    (text class=[TEXT_BASE]) {
        "hello world"
    }
}
```

If it is a component which has fields, you can also use fields as arguments:
```rust
#[derive(Component)]
pub struct MyComponent {
    pub i: u8,
    pub name: String,
}
bsml! {MyComponent;
    (text class=[TEXT_BASE]) {
        "index: {}, name: {}", i, name
    }
}
```

**limitations**:
- you cannot have arguments in strings:
  - **valid**: `(text) { "index: {}, name: {}", i, name }`
  - **invalid**: `(text) { "index: {i}, name: {name}" }`
- you can only have component fields as arguments, no expressions. This may change in the future.
  - **valid**: `(text) { "index: {}, name: {}", i, name }`
  - **invalid**: `(text) { "index: {}, name: {}", "0".parse::<u8>().unwrap(), "hello world" }`
 
**slot** is an element used in components to let the users of the component fill in content.
Nested elements in braces of `slot` element is the default content if not supplied by user.
```rust
#[derive(Component)]
pub struct Button;
bsml! {Button;
    (slot class=[BG_WHITE, hovered(BG_BLUE_400)]) {
        (text) { "I am button" } // default content
    }
}

#[derive(Component)]
pub struct ButtonList;
bsml! {ButtonList;
    (Button)                               // "I am button"
    (Button) { (text) { "hello world" } }  // "hello world"
}
```

#### 4. Spawning a UI element

You can directly spawn a UI element without defining a component.

```rust
use bevy_bsml::{bsml, SpawnBsml};

commands.spawn_bsml(bsml!((node class=[BG_BLUE_200, hovered(BG_BLUE_400)]) {
    (text) { "hello world" }
});
```

You can also spawn the UI component:
```rust
use bevy_bsml::{bsml, SpawnBsml};

#[derive(Component)]
pub struct NameCard {
    name: String,
}
bsml!(NameCard;
  (node class=[BG_BLUE_200, hovered(BG_BLUE_400)]) {
    (text) { "hello {}", name }
  }
);

commands.spawn_bsml(NameCard { name: "jack".to_owned() });
```

#### 5. Defining a Reusable Component

You can also define UI component that can be reused by other elements, by specifying component struct type followed by semicolon.

```rust
#[derive(Debug, Clone, Component)]
pub struct Menu;

bsml! {Menu;
    (node class=[W_FULL, H_FULL, JUSTIFY_CENTER, ITEMS_CENTER, BG_WHITE]) {
        (node class=[FLEX_COL, gap_y(20.0)]) {
            (MenuItem i={0} name={"Continue".to_owned()})
            (MenuItem i={1} name={"Setting".to_owned()})
            (MenuItem i={2} name={"Exit".to_owned()})
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct MenuItem {
    pub i: u8,
    pub name: String,
}

bsml! {MenuItem;
    (node
        class=[w_px(200.0), h_px(75.0), BG_BLUE_400, hovered(BG_BLUE_600), JUSTIFY_CENTER, ITEMS_CENTER]
    ) {
        (text class=[TEXT_BASE]) { "{}: {}", i, name }
    }
}

commands.spawn_bsml(Menu);
```

### To Be Continued

I'll finish the rest of documentation when I have time.
In the mean time, check the examples to see how things work!
