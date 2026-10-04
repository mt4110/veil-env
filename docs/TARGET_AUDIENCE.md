# Target audience

## Independent engineers

Engineers working across client projects who want to reduce plaintext `.env`
files and manual credential copying. The intended benefit is reducing accidental
persistence and Git inclusion, not eliminating information-leak risk.

## Platform engineers

Teams evaluating a common local secret-injection workflow for trusted developer
applications. Enterprise use also needs vault authorization, audit policy,
endpoint controls, credential rotation, and supported lifecycle behavior.

## Product validation

This prototype is not enterprise-ready. First validate whether the workflow
improves an actual development task over existing vault-native run commands.
Measure setup burden, task completion, and accidental persistence using dummy data.
Proceed to a real provider only if the workflow has a concrete benefit.
