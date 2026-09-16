# Superseded Notify Datom check cancellation

The first remote `test-datom` invocation for the Notify proposal was cancelled
by its owner before completion. That target runs only `generated_contract` and
does not include `tests/notify_datom.rs`; cancellation is not a test failure.

The replacement target is `test-notify-datom`, which runs the named Notify
integration test with the `datom` feature.
