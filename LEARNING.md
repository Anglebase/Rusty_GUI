# Learn `rusty_gui`

## Define yourself element type.

### traits `Drawable` & `EventListener`

The core concept of `rusty_gui` is to treat the interface view and interface logic as two independent parts,
respectively as two traits, namely `Drawable` and `EventListener`. If you want to define a new interface element type,
you need to implement these two traits for that type to define the behavior of that element.Based on the above
description, you can easily obtain the following element definition template:

```rust
struct MyElement {
    // ...
}

impl Drawable for MyElement {
    fn draw(&mut self, canvas: &mut Canvas) {
        // ...
    }
}

impl EventListener for MyElement {
    fn on_event(&mut self, event: &Event) {
        // ...
    }
    // ...
}
```

### trait `AsWindow`

But it's not complete yet. It requires a field of type `Window` to interact with the system's GUI interface. In
`rusty_gui`, this is achieved through trait `AsWindow`, which requires the defined type to have a field of type `Window`
or a field that can be converted to a `Window` type, and its mutable and immutable references can be obtained through
functions in trait `AsWindow`. And you must implement this trait before you implement `Drawable` and `EventListener` for
your element type because it is defined like this:

```rust
pub trait AsWindow {
    // ...
}

pub trait Drawable: AsWindow {
    // ...
}

pub trait EventListener: AsWindow {
    // ...
}
```

Here is an example of its complete implementation:

```rust
struct MyElement {
    this: Window,
    // ...
}

impl AsWindow for MyElement {
    fn as_window(&self) -> &Window {
        &self.this
    }

    fn as_window_mut(&mut self) -> &mut Window {
        &mut self.this
    }
}

impl Drawable for MyElement {
    fn draw(&mut self, canvas: &mut Canvas) {
        // ...
    }
}

impl EventListener for MyElement {
    fn on_event(&mut self, event: &Event) {
        // ...
    }
}
```

But it's too complicated. So, `rusty_gui` provides a macro `default_as_window!` to simplify the process of implementing
`AsWindow` for your element type. Here is an example:

```rust
struct MyElement {
    this: Window,
    // ...
}

default_as_window!(MyElement);

impl Drawable for MyElement {
    fn draw(&mut self, canvas: &mut Canvas) {
        // ...
    }
}

impl EventListener for MyElement {
    fn on_event(&mut self, event: &Event) {
        // ...
    }
}
```

### Constructor of your element type

Before it, you need to implement the trait `Default` for your element type to provide a default constructor. Then, you can define the function named `new` to provide a constructor for your element type. Here is an example:

```rust
struct MyElement {
    this: Window,
    // ...
}

default_as_window!(MyElement);

impl Default for MyElement {
    fn default() -> Self {
        Self {
            this: Window::default(),
            // ...
        }
    }
}

impl MyElement {
    pub fn new(title: &str, rect: Rect, parent: Option<&Window>) -> Widget<Self> {
        Widget::new(title, rect, parent)
    }
}
```

The function `Widget::new` will call the function `default` to construct an instance of your element type and return a `Widget` object.

### Use your element type

Now, you can create an instance of your element type and show it on the screen:

```rust
fn main() {
    let app = Applicaion::new(true);

    let elem: Widget<MyElement> = MyElement::new("MyElement", rect!(100, 100, 800, 600), None);
    elem.as_window().show();

    app.exec();
}
```

The function `exec` of `Application` type is used to start the event loop and process the events. When you call `show`
method of `Window` type, it will make the element visible on the screen and update its content.

## How to communicate with other elements.

`rusty_gui` provide a way to communicate with other elements through two structs: `Notifier` and `Responder`. They are
essentially an encapsulation of function callbacks. `Responder` is the wrapper of callback functions. `Notifier` holds a
list of `Responder` objects and a list of disable `Responder` objects. When method `notify` of `Notifier` is called, it
will call all the `Responder` but the disabled ones.

### Use `Notifier` & `Responder`

They are all generic types, theirs generic parameter is the type of the element that wants to communicate with other
elements. `Notifier` is usually used as a public field of the element struct. Here is an example:

Define a new element type `MyElement` with a `Notifier` field:

```rust
struct MyElement {
    this: Window,
    pub notifier: Notifier<i32>,
    // ...
}

default_as_window!(MyElement);

impl Drawable for MyElement {
    fn draw(&mut self, canvas: &mut Canvas) {
        // ...
    }
}

impl EventListener for MyElement {
    fn on_event(&mut self, event: &Event) {
        if let Event::MouseButtonPressed { button, .. } = event {
            if *button == MouseButton::Left {
                self.notifier.notify(&10);
            }
        }
        // ...
    }
}

impl Default for MyElement {
    fn default() -> Self {
        Self {
            this: Window::default(),
            notifier: Notifier::new(),
            // ...
        }
    }
}

impl MyElement {
    pub fn new() -> Widget<Self> {
        Widget::new("MyElement", rect!(100, 100, 800, 600), None)
    }
}
```

And use it in the main function:

```rust
fn main() {
    let app = Application::new(true);

    let mut elem = MyElement::new();
    elem.as_window().show();

    elem.notifier.add(
        "my_responder",
        Responder::new(|i: &i32| {
            println!("Got value: {}", i);
        })
    );

    app.exec();
}
```

Now, when you click the left mouse button on the element, it will call the callback function of the `Responder` named "
my_responder" with the value `10`. You can add more `Responder` objects by calling the `add` method of `Notifier`
object.

### `WindowID` and `Window::post`

Through the above method, you can achieve data broadcasting.But if you want to modify the data in the window instance,
this is inappropriate.`rusty_gui` provides the structure `WindowID` and associated function `Window::post`，This allows
the behavior of structures to be triggered through event mechanisms.

There is another function named `on_message` in trait `EventListener` that has a default empty implementation. If you
need to pass custom message data through `Window::post`, this function will be called. By defining the content of this
function, you can perform some modification operations.

Here is an example:

```rust
use rusty_gui::*;
use std::any::Any;
struct MyElement {
    this: Window,
    pub notifier: Notifier<i32>,
    // ...
    value: i32,
}

enum MyMessage {
    Add(i32),
}

default_as_window!(MyElement);

impl Drawable for MyElement {
    fn draw(&mut self, canvas: &mut Canvas) {
        canvas.clear(Color::WHITE);
        canvas.rect_text(
            self.this.rect(),
            &format!("value: {}", self.value),
            TextAlign::Center,
        );
    }
}

impl EventListener for MyElement {
    fn on_event(&mut self, event: &Event) {
        if let Event::MouseButtonPressed { button, .. } = event {
            if *button == MouseButton::Left {
                self.notifier.notify(&10);
            }
        }
        // ...
    }

    fn on_message(&mut self, message: Box<dyn Any>) {
        let message = *message.downcast::<MyMessage>().unwrap();
        match message {
            MyMessage::Add(i) => {
                self.value += i;
                self.this.update();
            }
        }
    }
}

impl Default for MyElement {
    fn default() -> Self {
        Self {
            this: Window::default(),
            notifier: Notifier::new(),
            value: 0,
            // ...
        }
    }
}

impl MyElement {
    pub fn new() -> Widget<Self> {
        Widget::new("MyElement", rect!(100, 100, 800, 600), None)
    }
}

fn main() {
    let app = Application::new(true);

    let mut elem = MyElement::new();
    elem.as_window().show();
    let id = elem.as_window().get_id();

    elem.notifier.add(
        "my_responder",
        Responder::new(move |i: &i32| {
            Window::post(id, MyMessage::Add(*i));
        }),
    );

    app.exec();
}
```

In this example, the `on_message` function of `MyElement` is defined to handle the custom message `MyMessage::Add`. When
the `Responder` named "my_responder" is called, it will post a message of type `MyMessage::Add` to the window instance
with the value of `i`. The `Window::post` function will call the `on_message` function of the window instance with the
message. In this case, the `on_message` function will add the value of `i` to the `value` field of `MyElement` and call
the `update` method of `Window` to update the content of the window.

## Expand on existing elements.