# Tackweld - Rust Virtual Dom 

Basic idea: if we break components down into "snippets"
we can avoid mixing logic and html.

## Theoretical usage:


__todos.html__

```html
<!-- Snippet "TodoPage" -->
<TodoPage>
    <div>
        {todo_items}
    </div>
</TodoPage>

<!-- Snippet "TodoItem" -->
<TodoItem>
    <span class="todo_num">{num}</span>
    <span class="todo_value">{val}</span>
</TodoItem>
```

__todos.rs__

```rust

fn todos(items: Vec<String>) -> TodoPage {
    
}

#[load_templates="src/**/*.html"]
struct Dummy;

// #[load_templates=".."] will generate
struct TodoPage<'a> {
    todo_items: &'a Renderable
}

struct TodoItem<'a> {
    num: &'a Renderable,
    val: &'a Renderable,
}
```