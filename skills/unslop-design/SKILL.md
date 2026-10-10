---
name: unslop-design
description: "Review or refine existing websites and UI for generic composition, weak hierarchy and unnecessary decoration while preserving brand, accessibility and behavior. Use for unslop design, unslop UI, or a focused visual cleanup."
---
Usable directly or through `wstack unslop-design`. Apply Wstack principles when
installed; this folder contains the whole design workflow.

<!-- wstack:begin shared/unslop-design.md -->
# Unslop design

Refine existing websites and UI so composition serves the content, task and
approved identity. Patterns are review prompts, never evidence of AI authorship.
Leave effective design alone, including familiar components and visual styles.

## Design pass

1. Inspect the actual result and relevant brand guidance. Identify the audience,
   primary task and requested scope. For editable UI, capture a useful baseline.
2. Find concrete problems below. Make the smallest changes that improve hierarchy,
   comprehension or fit. A review-only request produces findings, not mutations.
3. Inspect the changed result at relevant wide and narrow sizes. For interactive
   changes, exercise affected controls, keyboard focus and content/state variants.
   Source inspection alone cannot establish the visual result; disclose gaps.
4. Compare with the baseline. Preserve useful identity and behavior; stop when
   further edits would only substitute taste. Briefly explain consequential edits.

## Patterns and examples

- **Interchangeable heroes.** A generic slogan, stock dashboard and giant empty
  hero may hide the product. "Manage everything effortlessly" above unrelated
  cards -> a specific task statement and the actual product view, when useful.
  Keep a spacious hero when its message or imagery earns the space.
- **Cards around everything.** Containers should express relationships. Six
  separate cards for six related settings -> one labeled settings group. Keep
  cards for independently actionable items or useful side-by-side comparisons.
- **Equal emphasis everywhere.** Multiple filled buttons, oversized headings and
  highlighted panels can obscure the next action. Three competing primary
  buttons -> one primary action with quieter alternatives, if the task has one.
- **Decoration competing with content.** Glows, gradients, glass effects and
  badges need a role. A glow behind every metric -> emphasis on the metric that
  needs attention. Keep decoration that supports identity or understanding.
- **Redundant icons.** An icon beside every sentence adds noise without helping
  recognition. Remove repeated decorative sparkles; keep recognizable action or
  status icons, with accessible names and text where needed.
- **Forced grids and symmetry.** Do not invent items to fill a template. Five
  sequential steps in a three-column feature grid -> an ordered flow. Keep grids
  when equal items genuinely benefit from comparison; avoid arbitrary asymmetry.
- **Generic content driving layout.** Placeholder testimonials, invented metrics
  and empty feature claims create false polish. Replace a fabricated proof strip
  with supplied evidence, or remove it within scope. Do not invent social proof,
  product screens or assets; identify missing content if it blocks the design.
- **Weak information density.** Oversized gaps can separate labels from values
  or push the main task below decoration. Tighten a dashboard's metric grouping
  before shrinking all text. Preserve comfortable reading and touch targets.
- **False affordances and hidden actions.** A hover-styled statistic may look
  clickable; a real action may look like a caption. Make a static metric visibly
  static and give the action a recognizable control. Preserve keyboard access.
- **Decorative motion.** Repeated entrances or bouncing accents can delay reading
  and distract from a task. Remove animation that only reintroduces static content;
  retain feedback that explains a transition and respect reduced-motion settings.
- **Sample-only polish.** A layout may work only with short text and full data.
  Check realistic long labels, narrow widths and affected empty, loading and error
  states. Let a long product name wrap without hiding its price or action.

## Preserve

- Approved branding, intentional visual strengths, meaningful imagery and useful
  conventions. Fonts, purple gradients, rounded corners and centered layouts are
  context-dependent choices, not banned styles.
- Real content, accessibility, working controls and responsive behavior. Cosmetic
  cleanup must not remove labels, contrast, focus indicators or meaningful states.
- The requested scope. Review UI copy with the writing lens only when copy is
  included. New identity, new product behavior or missing content may need separate
  work; explain that dependency instead of expanding a cleanup into a redesign.

Every proposed change needs a concrete benefit for this artifact. Do not replace
generic design with forced novelty or strip away character to make it look plain.
<!-- wstack:end shared/unslop-design.md -->
