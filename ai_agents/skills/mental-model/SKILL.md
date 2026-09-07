---
name: mental-model
description: Improve the user's mental model of a subject through repeated explanation, evidence-based review, and revision. Use when the user wants to test their understanding against how something actually works, with HTML reports, SVG explanations, and interactive laboratories that build intuition.
---

# Mental model

Help the user build a model that explains and predicts the target subject. Treat their explanation as a hypothesis to test, not an answer to grade. Follow a scientific-method-inspired cycle of explanation, prediction, evidence, experiment, and revision.

Deliver every model review as a standalone HTML document, with inline CSS, SVG illustrations, and JavaScript for interactive laboratories. Link the document in your reply; a chat-only review is not a substitute. The document must work locally without external assets or dependencies.

## Establish the target

Ask for these together, unless the user already supplied them:

- The subject they want to understand and the scope that matters to them.
- A brief explanation, in their own words, of how they think it works. Invite uncertainty and guesses.

Do not teach the mechanism or offer a model answer before capturing their explanation. If the subject is too broad, help narrow it to a mechanism or question. Clarify ambiguous terms only when they change what would count as a correct explanation.

Agree on what the model should let the user explain or predict. For implementation-specific subjects, establish the relevant system, version, or configuration. Do not silently substitute textbook behavior for the user's actual target.

## Review the current model

1. Preserve the user's explanation faithfully. Separate what they said from your interpretation. An omitted detail is not proof of a misconception.
2. Extract the causal claims, assumptions, and predictions. When a claim is too vague to test, ask one focused question rather than inventing its meaning.
3. Check the claims against relevant evidence. Use source code and runnable checks for implementation behavior, official documentation for contracts, and primary research or established references for other subjects. Cite file paths and symbols or source links at the point of use.
4. Distinguish observed behavior, documented guarantees, inference, and unresolved questions. Do not present your own confidence as evidence. Report disagreements between sources and the limits of any test.
5. Classify claims as supported within scope, partly supported, contradicted, or unresolved. Separately identify missing mechanisms that matter to the agreed goal.
6. Choose the gaps that most affect prediction. Explain why each matters with a concrete case where the current model and the supported explanation produce different expectations.

Do not turn the review into a general textbook chapter. Preserve correct parts of the model and concentrate on the smallest changes that improve it.

## Track vocabulary and ubiquitous language

Words carry meaning. Treat vocabulary as part of the model, not a glossary added afterward. Build a shared, precise language for the agreed subject and use it consistently in explanations, SVG labels, and laboratories.

Maintain a vocabulary table in the HTML document. For each important term, record the user's intended meaning, the current shared definition, its scope, an example and a contrasting non-example, and whether its meaning is proposed, agreed, or unresolved. Ask when the user's meaning is unclear; do not silently replace it with your own.

Track ambiguous words, overloaded terms, synonyms, and distinct concepts the user has conflated. Prefer one agreed term per concept within scope. When the same word legitimately means different things in different contexts, name those contexts rather than forcing a single definition. Ground technical definitions in the same evidence used to review the model.

Update the table each round and record changed meanings and the reasons in that round's review. Preserve earlier usage in the learning history. Propose definitions for confirmation; do not label them agreed until the user demonstrates or confirms the shared meaning.

Test meaning through use, not memorization. Ask the user to distinguish neighboring concepts, classify a new example, or explain how a term affects a prediction. Separate a naming mismatch from a mistaken causal model, and address whichever the evidence shows.

## Write the HTML report

Use the project's documentation conventions. Otherwise, use `docs/mental-model/<subject-slug>.html` relative to the working project. With no working project, ask where to save it. Never put learning reports in the installed skill directory by default.

Maintain one document for the subject. Inspect an existing document before editing. Append one HTML section per review round, with an ID such as `round-01`, preserving earlier explanations and findings. Use stable, document-unique element IDs for the review, diagrams, and laboratories. Correct an earlier factual error explicitly rather than silently rewriting the learning history.

Each round contains:

- The target, scope, and the user's current explanation.
- A concise claim review showing what holds, what fails, what is missing, and what remains uncertain, with evidence.
- The corrected causal explanation for the selected gaps. Compare it directly with the user's explanation.
- Vocabulary changes, unresolved meanings, and a link to the current shared vocabulary table.
- SVG concept illustrations and interactive Laboratories chosen for those gaps.
- A brief revision prompt and an unworked transfer question.
- After the first round, what improved and what still needs testing.

Keep the report readable without running a Laboratory. Expand uncommon terms and label the limits of analogies.

### SVG illustrations

Create accessible inline SVGs when spatial structure, state, sequence, or relationships clarify a selected gap. Show the mechanism, not decorative imagery. Label entities, arrows, units, and boundaries as applicable. Include a text explanation of what the reader should notice. Do not rely on color alone.

### Interactive laboratories

Create real HTML/CSS/JavaScript Laboratories when manipulating inputs can reveal a mechanism. Prefer one focused experiment per important gap over a large simulator. Scope each laboratory's styles and DOM queries to its container, and keep its JavaScript variables local. Keep code local and dependency-free; do not make network requests or execute user-supplied code.

Each Laboratory must:

- Name the question being tested and the misconception it addresses.
- Ask the user to predict an outcome before revealing it. Provide an explicit run or reveal action.
- Offer labeled controls for meaningful inputs, visible outputs, and a reset action.
- Make cause and effect inspectable through intermediate states, a trace, or a visual update when useful.
- Include a discriminating case where the competing explanations predict different outcomes.
- Explain the result after execution and state assumptions, units, and simplifications.
- Support keyboard operation and make results understandable without color or animation alone.

A simulation illustrates its programmed assumptions. It is not independent evidence that the target works that way. Tie its rules to the cited evidence and distinguish simulated results from actual measurements. If the subject does not support a meaningful simulation, use an interactive case comparison or prediction exercise. Do not invent quantitative behavior to justify a Laboratory.

Validate non-trivial experiment logic with at least one small runnable check against independently established expectations. Check a normal case and the misconception-revealing case. Follow the user's Taskfile conventions when generating executable code. In a browser, test controls, reset behavior, displayed results, and console errors when tools are available. Report exactly what was checked and any unavailable validation. Valid HTML alone does not prove the experiment works.

Do not force a diagram or Laboratory where it adds no explanatory value. Explain the omission rather than adding filler.

## Ask for the next model

Link the report and briefly name the main gap to investigate. Ask the user to read it, try the experiments, then explain the mechanism again in their own words. Request a prediction for one new case, with reasoning, and invite remaining uncertainty. Ask them to use the shared vocabulary and test any important term whose meaning remains unclear.

Leave the transfer question unanswered until they attempt it. Do not generate their revised explanation for them or continue to the next round without their response.

On the next attempt, compare it with both the evidence and the previous model. Test whether they can use the mechanism in a new situation, not merely repeat the report's vocabulary. If a misconception persists, change the example or experiment instead of repeating the same explanation more loudly.

## Decide when to stop

The model is adequate for the agreed scope when the user can explain the relevant mechanism, predict an unfamiliar case with sound reasoning, and identify important limits or unknowns, with no material contradiction left unresolved. It must also have a clear ubiquitous language: the user and agent share explicit meanings for the important terms, use them consistently, and can distinguish concepts in new examples. No ambiguity that materially affects explanation or prediction may remain unresolved.

State what has been demonstrated and what remains outside scope. Do not claim complete understanding or assign unsupported mastery scores. If evidence is insufficient, name the unresolved question rather than marking it correct. Continue while the user wants to refine the model; respect a pause or request to stop.
