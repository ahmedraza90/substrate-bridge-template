pub	Anyone  (fully public)
pub(super)	Only parent module
pub(crate)	Anywhere in the crate
(no pub)	Only this module


CONFUSION:

QUESTION:
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}
// when we have already write pub with struct name then why are we writting pub keyword with each field of struct... is it not enough to write pub with struct name only

ANSWER:
// In Rust, writing pub before the struct name (e.g., pub struct TaskResponse) makes the struct itself public, meaning other modules can use the type.
// However, fields inside the struct are private by default, even if the struct is public.
// So, if you want other modules to access the fields directly (e.g., response.task), you must also write pub before each field:

qualifiers with implementations:
For inherent impls (e.g., impl MyType { ... }), you can use visibility qualifiers like pub on methods and fields. This controls whether those methods are accessible from outside the module.
For trait impls (e.g., impl SomeTrait for MyType { ... }), you must NOT use pub on the methods. The visibility is determined by the trait definition itself, not by the impl.