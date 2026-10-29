# mado-openrouter-models

Hide OpenRouter models from Pi's `/model` and `/models` pickers by default. OpenAI subscription models remain visible. This extension does not remove OpenRouter credentials or edit `models.json`.

Load it for a Pi session:

```sh
pi --extension /path/to/ai_agents/pi.dev/extensions/mado-openrouter-models/index.ts
```

For every session, copy `index.ts` into `~/.pi/agent/extensions/mado-openrouter-models.ts` instead. Run `/reload` after installing it in an open Pi session.

- `/mado-openrouter-models on` shows OpenRouter models again.
- `/mado-openrouter-models off` hides them again.
- `/mado-openrouter-models status` checks their visibility.
- `/mado-openrouter-models` toggles between on and off.

Reopen `/models` after changing visibility. If an OpenRouter model is active, switch to another model before hiding it. Visibility resets to hidden on a new session; if another extension manages the OpenRouter provider, this extension leaves it alone.
