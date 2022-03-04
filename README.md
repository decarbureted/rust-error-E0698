# rust-error-E0698
This repo contains a minimal example to reproduce and investigate an unexpected rust compiler error E0698 when attempting to implement an async upsert wrapper function for a generic type.

To replicate the issue, run `cargo build`.

This will yield the follwing error:

```
   Compiling rust-error-E0698 v0.1.0 (/home/dwputney/devel/rust-error-E0698)
error[E0698]: type inside `async fn` body must be known in this context
   --> src/main.rs:129:64
    |
129 |         let binding_collection = client.database(&T::DATABASE).collection(&T::COLLECTION);
    |                                                                ^^^^^^^^^^ cannot infer type for type parameter `T` declared on the associated function `collection`
    |
note: the type is part of the `async fn` body because of this `await`
   --> src/main.rs:133:29
    |
133 |         let upsert_result = binding_collection.update_one(query, d, None).await;
    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0698]: type inside `async fn` body must be known in this context
   --> src/main.rs:129:64
    |
129 |         let binding_collection = client.database(&T::DATABASE).collection(&T::COLLECTION);
    |                                                                ^^^^^^^^^^ cannot infer type for type parameter `T`
    |
note: the type is part of the `async fn` body because of this `await`
   --> src/main.rs:133:29
    |
133 |         let upsert_result = binding_collection.update_one(query, d, None).await;
    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0698`.
error: could not compile `rust-error-E0698` due to 2 previous errors
```