# signal-message architecture

Vocabulary only: no runtime, no sockets, no store.

- `Priority.[ HardAbrupt MiddleAbrupt Soft ]` is the head the recipient sees:
  Message hands Flow a `Message` value whose variant is the Priority.
- `Grade` keeps every receipt grade separate: Submitted, Parked, Transported,
  Presented, Uncertain, Read, Withdrawn, Refused (carrying Flow's own
  `DeliveryRejection`). No grade is ever upgraded into another.
- The sender is not a field of any request. It is stamped by the Nexus from
  the peer.

Retired in 6.0.0: the agent registry, inbox, threads, the cluster relay and
peer envelope, stamped submission, and the flow-delivery park. Flow holds
identity; `Content.Psyche` carries the psyche's words.
